from fastapi import HTTPException
from interface import ServiceInfo, TaskRequest, ServiceReport, HeartbeatData, TaskResult
import redis
from json import loads as json_loads
import time

class RedisBridge:
    # private
    _REGISTRY_KEY = "service_registry"
    _REQ_KEY = "requests"
    _RES_KEY = "responses"
    _HEARTBEAT_KEY = "heartbeat"
    _STATUS_KEY = "status"
    def hset(self, name, key, value):
        return self.client.hset(name, key, value)
    def hget(self, name, key):
        return self.client.hget(name, key)
    def hdel(self, name, key):
        return self.client.hdel(name, key)
    def hgetall(self, name):
        return self.client.hgetall(name)
    def xadd(self, name, fields):
        return self.client.xadd(name, fields)
    def hsetex(self, name, key, value, ex=None):
        return self.client.hsetex(name, key, value, ex=ex)
    def hexists(self, name, key):
        return self.client.hexists(name, key)
    def hkeys(self, name):
        return self.client.hkeys(name)
    def hvals(self, name):
        return self.client.hvals(name)

    # constructor
    def __init__(self, host="host.docker.internal", port=16379, password="alice_margatroid", decode_responses=True):
        self.client = redis.Redis(host=host, port=port, password=password, decode_responses=decode_responses)
    
    # public, for services
    def register_service(self, info: ServiceInfo):
        svc_id = f"{info.name}:{info.version}"
        info_json = info.model_dump_json()
        self.hset(self._REGISTRY_KEY, svc_id, info_json)
        return {"status": "registered"}
    def unregister_service(self, service_name: str, service_version: str):
        svc_id = f"{service_name}:{service_version}"
        if not self.hexists(self._REGISTRY_KEY, svc_id):
            raise HTTPException(404, "Service not found")
        self.hdel(self._REGISTRY_KEY, svc_id)
        return {"status": "unregistered"}
    def service_heartbeat(self, service_name: str, service_version: str, data: HeartbeatData):
        svc_id = f"{service_name}:{service_version}"
        self.hsetex(self._HEARTBEAT_KEY, svc_id, data.timestamp, ex=data.ttl)  # Set expiration for heartbeat, value is timestamp
        return {"status": "alive"}
    def update_service(self, info: ServiceInfo):
        svc_id = f"{info.name}:{info.version}"
        info_json = info.model_dump_json()
        if not self.hexists(self._REGISTRY_KEY, svc_id):
            raise HTTPException(404, "Service not found")
        self.hset(self._REGISTRY_KEY, svc_id, info_json)
        return {"status": "updated"}
    def service_report(self, service_name: str, service_version: str, report: ServiceReport):
        svc_id = f"{service_name}:{service_version}"
        if not self.hexists(self._STATUS_KEY, svc_id):
            raise HTTPException(404, "Service not found")
        self.hset(self._STATUS_KEY, svc_id, report.model_dump_json())
        return {"status": "report received", "svc_id": svc_id}
    def pending_tasks(self, service: str, version: str):
        svcq_id = f"{self._REQ_KEY}:{service}:{version}"
        return self.hgetall(svcq_id)  # {req_id: payload}
    def store_result(self, service: str, version: str, req_id: str, result: TaskResult):
        res_id = f"{self._RES_KEY}:{service}:{version}"
        print(result)
        self.hset(res_id, req_id, result.model_dump_json())
        svcq_id = f"{self._REQ_KEY}:{service}:{version}"
        if self.hexists(svcq_id, req_id):
            self.hdel(svcq_id, req_id)
        return {"status": "stored", "request_id": req_id}
    
    # public, for users
    def service_info(self, service_name: str, service_version: str):
        svc_id = f"{service_name}:{service_version}"
        if not self.hexists(self._REGISTRY_KEY, svc_id):
            raise HTTPException(404, "Service not found")
        info = self.hget(self._REGISTRY_KEY, svc_id)
        return json_loads(info)
    def list_services(self):
        services = self.hgetall(self._REGISTRY_KEY)
        return {k: json_loads(v) for k, v in services.items()}
    def submit_task(self, task: TaskRequest):
        timestamp = int(time.time() * 1000)
        svcq_id = f"{self._REQ_KEY}:{task.service}:{task.version}" # one queue per service - version pair
        req_id = f"{task.user_id}:{timestamp}"
        self.hset(svcq_id, req_id, task.payload)
        return {"status": "submitted", "request_id": req_id}
    def task_result(self, service: str, service_version: str, req_id: str):
        res_id = f"{self._RES_KEY}:{service}:{service_version}"
        if not self.hexists(res_id, req_id):
            raise HTTPException(404, "No results found")
        res = self.hget(res_id, req_id)
        self.hdel(res_id, req_id)  # Remove result after fetching
        return json_loads(res)