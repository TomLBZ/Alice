# runs the test docker container headless
# this docker container will run foreever and the script should not wait for it

# a temmporary variable to store the name of the current folder
# this is used to name the docker container
cname=$(pwd | xargs basename)
cnamelower=$(echo $cname | tr '[:upper:]' '[:lower:]')
# run the docker container
docker run -d --name $cnamelower $cnamelower