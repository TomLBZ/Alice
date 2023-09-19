# this python script is run when the backend container is started
# it first generates a ramdom ID as the "nodeID", and writes it to the file nodeID
# then it starts the backend api server contained in simpleapi.py

# imports
import os
import random
import string
import subprocess
import psycopg2

# database connection
conn = psycopg2.connect(
    host="localhost",
    database="postgres",
    user="postgres",
    password="Lbz_142857",
    port="15432"
)

# generate nodeID if it does not exist
if not os.path.exists("nodeID"):
    nodeID = ''.join(random.choices(string.digits, k=8))
    # write nodeID to file
    with open("nodeID", "w") as f:
        f.write(nodeID)
else:
    # read nodeID from file
    with open("nodeID", "r") as f:
        nodeID = f.read()

# print nodeID
print("nodeID:", nodeID)

# creates a table in the database named nodeID if it does not exist
# this table is used to store entries related to this node identified by nodeID
# table columns: id, name, description, endpoint
cur = conn.cursor()
cur.execute("CREATE TABLE IF NOT EXISTS info" + nodeID + " (id SERIAL PRIMARY KEY, name TEXT, description TEXT, endpoint TEXT)")
conn.commit()

# creates a table in the database named f"{nodeID}status" if it does not exist
# this table is used to store entries related to the status of each node entry
# table columns: id, state, instances, calls, errors, latency, ram, uptime
cur.execute("CREATE TABLE IF NOT EXISTS status" + nodeID + " (id SERIAL PRIMARY KEY, state BOOLEAN, instances INTEGER, calls INTEGER, errors INTEGER, latency INTEGER, ram INTEGER, uptime INTEGER)")
conn.commit()

# check whether there is id=nodeID in the database table nodeID
cur.execute("SELECT * FROM info" + nodeID + " WHERE id=" + nodeID)
rows = cur.fetchall()
if len(rows) == 0:
    # create a new entry in the database table nodeID
    cur.execute("INSERT INTO info" + nodeID + " (id, name, description, endpoint) VALUES (" + nodeID + ", 'Shelf', 'The Web Application Core', 'http://localhost:8000')")
    # create a new entry in the database table f"{nodeID}status"
    cur.execute("INSERT INTO status" + nodeID + " (id, state, instances, calls, errors, latency, ram, uptime) VALUES (" + nodeID + ", TRUE, 1, 0, 0, 0, 0, 0)")
    conn.commit()

# close database connection
conn.close()

# start backend api server
subprocess.run(["python3", "simpleapi.py"])