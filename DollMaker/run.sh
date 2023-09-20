# runs the test docker container headless
# this docker container will run foreever and the script should not wait for it

# a temmporary variable to store the name of the current folder
# this is used to name the docker container
cname=$(pwd | xargs basename)
cnamelower=$(echo $cname | tr '[:upper:]' '[:lower:]')

# check if a docker container with the same name is already running
if [ "$(docker ps -q -f name=$cnamelower)" ]; then
    # stop the docker container
    docker stop $cnamelower
fi

# check if a docker with the same name exists
if [ "$(docker ps -aq -f status=exited -f name=$cnamelower)" ]; then
    # remove the docker container
    docker rm $cnamelower
fi

# run the docker container
docker run -d --name $cnamelower $cnamelower