import requests
import time
import io
import contextlib
import traceback

# === Configuration ===
SERVICE_NAME = "python_interpreter"
SERVICE_VERSION = "1.0.0"
API_BASE = "http://localhost:18000"
STREAMING_MODE = "none"  # no streaming response
HEARTBEAT_TTL = 10  # seconds

# === Service ID & Endpoints ===
SERVICE_ID = f"{SERVICE_NAME}:{SERVICE_VERSION}"

# === Register with Gateway ===
def register():
    service_info = {
        "name": SERVICE_NAME,
        "version": SERVICE_VERSION,
        "input_type": "text/python",
        "output_type": "text/log",
        "request_stream": f"requests:{SERVICE_ID}",
        "response_stream_base": f"responses:{SERVICE_ID}",
        "description": "Executes Python code snippets.",
        "streaming_mode": STREAMING_MODE
    }
    response = requests.post(f"{API_BASE}/service/register", json=service_info)
    if response.status_code != 200:
        print("Failed to register service:", response.status_code, response.text)
    else:
        print("Registered:", response.json())
# === Send Heartbeat ===
def send_heartbeat():
    heartbeat_data = {
        "ttl": HEARTBEAT_TTL,
        "timestamp": int(time.time() * 1000)  # Current timestamp in milliseconds
    }
    response = requests.post(f"{API_BASE}/service/heartbeat/{SERVICE_NAME}/{SERVICE_VERSION}", json=heartbeat_data)
    if response.status_code != 200:
        print("Failed to send heartbeat:", response.status_code, response.text)
    else:
        print("Heartbeat sent:", response.json())
# === Poll Gateway for New Tasks ===
def poll_tasks():
    response = requests.get(f"{API_BASE}/task/pending/{SERVICE_NAME}/{SERVICE_VERSION}")
    if response.status_code != 200:
        print("Failed to fetch tasks:", response.status_code, response.text)
        return []
    tasks = response.json()
    if not tasks:
        return []
    return [(req_id, code) for req_id, code in tasks.items()]

# === Execute Python Code ===
def run_code(code: str) -> str:
    result = ""
    error = ""
    try:
        stdout = io.StringIO()
        with contextlib.redirect_stdout(stdout):
            exec(code, {}, {})
        result = stdout.getvalue()
    except Exception:
        error = "ERROR:\n" + traceback.format_exc()
    return result, error

# === Fetch & Handle Task ===
def process_task(req_id: str, code: str):
    print(f"Processing task: {req_id}")
    result, error = run_code(code)
    # Post result back to API
    response_payload = {
        "timestamp": int(time.time() * 1000),  # Current timestamp in milliseconds
        "result": result,
        "error": error
    }
    requests.post(f"{API_BASE}/task/result/{SERVICE_NAME}/{SERVICE_VERSION}/{req_id}", json=response_payload)
    print("Result submitted")

# === Main Loop (placeholder task simulation) ===
def main_loop():
    heartbeat_cycle = 1
    while True:
        try:
            heartbeat_cycle += 1
            if heartbeat_cycle == HEARTBEAT_TTL:
                heartbeat_cycle = 1
                send_heartbeat()
                print("Heartbeat sent.")
            tasks = poll_tasks()
            if tasks:
                for req_id, code in tasks:
                    process_task(req_id, code)
        except Exception as e:
            print("Error in main loop:", e)
        time.sleep(1)

# === Entry Point ===
if __name__ == "__main__":
    register()
    try:
        main_loop()
    except KeyboardInterrupt:
        print("Service shutting down.")