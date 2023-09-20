cname=$(pwd | xargs basename)
cnamelower=$(echo $cname | tr '[:upper:]' '[:lower:]')
docker stop $cnamelower