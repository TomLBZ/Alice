from fastapi import FastAPI
from interface import ServiceInfo, TaskRequest, ServiceReport, HeartbeatData, TaskResult
from bridge.redis import RedisBridge

app = FastAPI()
r = RedisBridge(host="host.docker.internal", port=16379, password="alice_margatroid", decode_responses=True)

# for services
@app.post("/service/register")
def register_service(info: ServiceInfo):
    return r.register_service(info)
@app.post("service/update")
def update_service(info: ServiceInfo):
    return r.update_service(info)
@app.get("/service/unregister/{service_name}/{service_version}")
def unregister_service(service_name: str, service_version: str):
    return r.unregister_service(service_name, service_version)
@app.post("/service/heartbeat/{service_name}/{service_version}")
def service_heartbeat(service_name: str, service_version: str, data: HeartbeatData):
    return r.service_heartbeat(service_name, service_version, data)
@app.post("/service/report/{service_name}/{service_version}")
def service_report(service_name: str, service_version: str, report: ServiceReport):
    return r.service_report(service_name, service_version, report)
@app.get("/task/pending/{service}/{service_version}")
def pending_tasks(service: str, service_version: str):
    return r.pending_tasks(service, service_version)
@app.post("/task/result/{service}/{service_version}/{req_id}")
def post_task_result(service: str, service_version: str, req_id: str, result: TaskResult):
    return r.store_result(service, service_version, req_id, result)

# for users
@app.get("/service/info/{service_name}/{service_version}")
def service_info(service_name: str, service_version: str):
    return r.service_info(service_name, service_version)
@app.get("/services")
def list_services():
    return r.list_services()
@app.post("/task/submit")
def submit_task(task: TaskRequest):
    return r.submit_task(task)
@app.get("/task/result/{service}/{service_version}/{req_id}")
def task_result(service: str, service_version: str, req_id: str):
    return r.task_result(service, service_version, req_id)
@app.get("/test")
def test_endpoint():
    return {"message": "Test endpoint is working!"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("app:app", host="0.0.0.0", port=8000, reload=True)