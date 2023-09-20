cname=$(pwd | xargs basename)
cnamelower=$(echo $cname | tr '[:upper:]' '[:lower:]')
docker build -t $cnamelower .