from pydantic import BaseModel

class ServiceInfo(BaseModel):
    name: str
    version: str
    input_type: str
    output_type: str
    request_stream: str
    response_stream_base: str
    description: str
    streaming_mode: str  # "none", "response", "bidirectional"

class TaskRequest(BaseModel):
    service: str
    version: str = "latest"  # Default to latest version
    user_id: str
    payload: str

class ServiceReport(BaseModel):
    timestamp: int
    status: str

class HeartbeatData(BaseModel):
    ttl: int  # Time to live in seconds
    timestamp: int  # Current timestamp in milliseconds

class TaskResult(BaseModel):
    timestamp: int
    result: str  # The result of the task execution, serialized as a string
    error: str  # Optional error message if the task failed