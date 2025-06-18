back_address = "http://localhost:18000"
import requests
from pydantic import BaseModel
from time import sleep

class ServiceInfo(BaseModel):
    name: str
    version: str
    input_type: str
    output_type: str
    request_stream: str
    response_stream_base: str
    description: str
    streaming_mode: str  # "none", "response", "bidirectional"

def list_available_services():
    response = requests.get(f"{back_address}/services")
    assert response.status_code == 200
    services = response.json()
    for name, info in services.items():
        print(f"{name}: accepts {info['input_type']}, returns {info['output_type']}")
    return services.items()

def test_post_task():
    task = {
        "service": "python_interpreter",
        "version": "1.0.0",
        "user_id": "test_user",
        "payload": "print('Hello, World!')"
    }
    response = requests.post(f"{back_address}/task/submit", json=task)
    assert response.status_code == 200
    response_data = response.json()
    assert "request_id" in response_data
    return response_data["request_id"]

def get_task_result(service: str, version: str, req_id: str):
    response = requests.get(f"{back_address}/task/result/{service}/{version}/{req_id}")
    assert response.status_code == 200
    return response.json()

if __name__ == "__main__":
    items = list_available_services()
    print("Available services listed successfully.")
    namever, info = next(iter(items))
    name, version = namever.split(":")
    print(f"Testing service {name} version {version}...")
    assert info["name"] == name
    assert info["version"] == version
    req_id = test_post_task()
    print("Task submission test passed.")
    sleep(1)  # Give some time for the task to be processed
    result = get_task_result(name, version, req_id)
    print("Task result fetched successfully:", result)