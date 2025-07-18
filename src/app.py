# host="host.docker.internal", port=16379, password="alice_margatroid", decode_responses=True
from fastapi import FastAPI, File, Form, UploadFile, WebSocket, WebSocketDisconnect, BackgroundTasks, Request
from fastapi.responses import JSONResponse, HTMLResponse
from fastapi.staticfiles import StaticFiles
import os
import uuid
import subprocess
import asyncio
import shutil
import redis
import json
from datetime import datetime
from typing import Awaitable

app = FastAPI()
app.mount("/static", StaticFiles(directory="static"), name="static")
rdb = redis.Redis(host="host.docker.internal", port=16379, password="alice_margatroid", decode_responses=True)

SERVICE_DIR = "./services"
os.makedirs(SERVICE_DIR, exist_ok=True)

@app.get("/")
async def root():
    with open("static/index.html") as f:
        return HTMLResponse(f.read())

@app.get("/services")
def get_all_services():
    services = rdb.hgetall("service_registry")
    now = int(datetime.now().timestamp())
    result = {}
    assert isinstance(services, dict), "Expected services to be a dictionary"
    for k, v in services.items():
        info = json.loads(v)
        created_at = info.get("created_at", now)
        uptime = now - created_at
        info["uptime"] = uptime
        result[k] = info
    return result

@app.post("/service/create")
async def create_service(
    name: str = Form(...),
    version: str = Form(...),
    description: str = Form(...),
    mode: str = Form(...),  # "function" or "service"
    exec_file: UploadFile = File(...)
):
    service_id = f"{name}:{version}"
    service_path = os.path.join(SERVICE_DIR, f"{service_id}_{uuid.uuid4().hex}")
    with open(service_path, "wb") as f:
        shutil.copyfileobj(exec_file.file, f)
    os.chmod(service_path, 0o755)

    now = int(datetime.now().timestamp())
    info = {
        "name": name,
        "version": version,
        "description": description,
        "mode": mode,
        "exec_path": service_path,
        "created_at": now,
        "sessions": 0,
        "uptime": 0
    }
    rdb.hset("service_registry", service_id, json.dumps(info))
    return {"status": "registered", "websocket_url": f"/ws/serve/{name}/{version}"}

@app.post("/service/update")
async def update_service(
    name: str = Form(...),
    version: str = Form(...),
    new_name: str = Form(None),
    new_version: str = Form(None),
    description: str = Form(...),
    mode: str = Form(...),
    exec_file: UploadFile = File(None)
):
    service_id = f"{name}:{version}"
    data = rdb.hget("service_registry", service_id)
    if isinstance(data, Awaitable):
        data = await data
    if not data:
        return JSONResponse(status_code=404, content={"error": "Service not found"})
    info = json.loads(data)
    info["description"] = description
    info["mode"] = mode

    if exec_file:
        old_path = info.get("exec_path")
        if old_path and os.path.exists(old_path):
            os.remove(old_path)
        service_path = os.path.join(SERVICE_DIR, f"{service_id}_{uuid.uuid4().hex}")
        with open(service_path, "wb") as f:
            shutil.copyfileobj(exec_file.file, f)
        os.chmod(service_path, 0o755)
        info["exec_path"] = service_path

    new_id = f"{new_name or name}:{new_version or version}"
    info["name"] = new_name or name
    info["version"] = new_version or version
    rdb.hdel("service_registry", service_id)
    rdb.hset("service_registry", new_id, json.dumps(info))

    return {"status": "updated", "new_id": new_id}

@app.post("/service/delete")
async def delete_service(name: str = Form(...), version: str = Form(...)):
    service_id = f"{name}:{version}"
    if not rdb.hexists("service_registry", service_id):
        return JSONResponse(status_code=404, content={"error": "Service not found"})
    data = rdb.hget("service_registry", service_id)
    if isinstance(data, Awaitable):
        data = await data
    if not data:
        return JSONResponse(status_code=404, content={"error": "Service not found"})
    info = json.loads(data)
    exec_path = info.get("exec_path")
    if exec_path and os.path.exists(exec_path):
        os.remove(exec_path)
    rdb.hdel("service_registry", service_id)
    return {"status": "deleted"}

@app.websocket("/ws/serve/{name}/{version}")
async def serve_service(name: str, version: str, websocket: WebSocket):
    await websocket.accept()
    service_id = f"{name}:{version}"
    data = rdb.hget("service_registry", service_id)
    if isinstance(data, Awaitable):
        data = await data
    if not data:
        await websocket.close(code=4404)
        return

    info = json.loads(data)
    exec_path = info["exec_path"]
    mode = info.get("mode", "function")
    info["sessions"] += 1
    rdb.hset("service_registry", service_id, json.dumps(info))

    try:
        if mode == "function":
            await handle_function_mode(websocket, exec_path)
        else:
            await handle_service_mode(websocket, exec_path)
    except WebSocketDisconnect:
        pass
    finally:
        data = rdb.hget("service_registry", service_id)
        if isinstance(data, Awaitable):
            data = await data
        if data:
            info = json.loads(data)
            info["sessions"] = max(0, info["sessions"] - 1)
            rdb.hset("service_registry", service_id, json.dumps(info))

async def handle_function_mode(websocket, exec_path):
    while True:
        input_data = await websocket.receive_text()
        proc = await asyncio.create_subprocess_exec(
            exec_path,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE
        )
        stdout, stderr = await proc.communicate(input=input_data.encode())
        if stderr:
            await websocket.send_text(stderr.decode())
        else:
            await websocket.send_text(stdout.decode())

async def handle_service_mode(websocket, exec_path):
    proc = await asyncio.create_subprocess_exec(
        exec_path,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )

    async def read_stdout():
        while True:
            if proc.stdout is None:
                break
            line = await proc.stdout.readline()
            if line:
                await websocket.send_text(line.decode())
            else:
                break

    reader = asyncio.create_task(read_stdout())

    try:
        while True:
            input_data = await websocket.receive_text()
            if proc.stdin:
                proc.stdin.write(input_data.encode() + b"\n")
                await proc.stdin.drain()
    except WebSocketDisconnect:
        reader.cancel()
        proc.terminate()
        await proc.wait()

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("app:app", host="0.0.0.0", port=8000, reload=True, reload_excludes=["__pycache__", "venv/*", "tmp/*", "*.pyc", "*.pyo", "*.pyd"])