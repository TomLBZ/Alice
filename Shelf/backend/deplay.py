# this python script is run when the backend container is started
# it first generates a ramdom ID as the "nodeID", and writes it to the file nodeID
# then it starts the backend api server contained in simpleapi.py

# imports
import os
import random
import string
import subprocess

# generate nodeID if it does not exist
if not os.path.exists("nodeID"):
    nodeID = ''.join(random.choices(string.ascii_uppercase + string.digits, k=8))
    # write nodeID to file
    with open("nodeID", "w") as f:
        f.write(nodeID)
else:
    # read nodeID from file
    with open("nodeID", "r") as f:
        nodeID = f.read()

# creates a table in the database named nodeID if it does not exist
# this table is used to store entries related to this node identified by nodeID
# table columns: id, name, description, endpoint
subprocess.run(["psql", "-h", "localhost", "-p", "15432", "-U", "postgres", "-c", "CREATE TABLE IF NOT EXISTS " + nodeID + " (id SERIAL PRIMARY KEY, name TEXT, description TEXT, endpoint TEXT)"])

# creates a table in the database named f"{nodeID}status" if it does not exist
# this table is used to store entries related to the status of each node entry
# table columns: id, state, instances, calls, errors, latency, ram, uptime
subprocess.run(["psql", "-h", "localhost", "-p", "15432", "-U", "postgres", "-c", "CREATE TABLE IF NOT EXISTS " + nodeID + "status (id SERIAL PRIMARY KEY, state TEXT, instances INTEGER, calls INTEGER, errors INTEGER, latency INTEGER, ram INTEGER, uptime INTEGER)"])

# start backend api server
subprocess.run(["python3", "simpleapi.py"])