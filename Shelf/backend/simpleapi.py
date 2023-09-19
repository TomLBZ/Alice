# this file serves api at http://localhost:8000/api/open/json
# it is used by the frontend to get data from the database
# the database is postgresql at localhost:15432

# imports
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import psycopg2

# database connection
conn = psycopg2.connect(
    host="localhost",
    database="postgres",
    user="postgres",
    password="Lbz_142857",
    port="15432"
)

# http server
class SimpleHTTPRequestHandler(BaseHTTPRequestHandler):
    # disable CORS
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        BaseHTTPRequestHandler.end_headers(self)

    def do_GET(self):
        # switch path
        if self.path == "/api/open/nodeID":
            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            # nodeID is read from file
            with open("nodeID", "r") as f:
                self.wfile.write(f.read().encode())
            return

        # check path
        elif self.path != "/api/open/json":
            self.send_response(404)
            return

        # set headers
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()

        # get nodeID from file
        with open("nodeID", "r") as f:
            nodeID = f.read()

        # get data from database
        # "nodeID" table columns: id, name, description, endpoint
        # f"{nodeID}status" table columns: id, state, instances, calls, errors, latency, ram, uptime
        # can join them using id
        cur = conn.cursor()
        cur.execute(f"SELECT info{nodeID}.id, name, description, endpoint, \
            state, instances, calls, errors, latency, ram, uptime \
            FROM info{nodeID} INNER JOIN status{nodeID} ON info{nodeID}.id = status{nodeID}.id")
        rows = cur.fetchall()

        # create json response
        response = []
        for row in rows:
            response.append({
                "id": row[0],
                "name": row[1],
                "description": row[2],
                "endpoint": row[3],
                "state": row[4],
                "instances": row[5],
                "calls": row[6],
                "errors": row[7],
                "latency": row[8],
                "ram": row[9],
                "uptime": row[10]
            })

        # send response
        self.wfile.write(json.dumps(response).encode())

# run http server
httpd = HTTPServer(('localhost', 8000), SimpleHTTPRequestHandler)
httpd.serve_forever()

# close database connection
conn.close()