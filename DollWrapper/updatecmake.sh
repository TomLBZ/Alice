#! /bin/bash

# for all .c and .h files in ./src, generate a string looks like this:
# "${CMAKE_SOURCE_DIR}/main.h", where main.h is the file name
# and put them between the line "add_executable(${PROJECT_NAME} " and the line ")" in CMakeLists.txt

# get paths
SRC_PATH="./src"
CMAKELISTS_PATH="CMakeLists.txt"

# get all .cpp files
FILES=$(ls $SRC_PATH | grep -E ".cpp")

# generate the string
STR=""
for FILE in $FILES
do
    STR="${STR}    \${CMAKE_SOURCE_DIR}/${FILE}\n"
done

# the existing lines may look like this:
# add_executable(${PROJECT_NAME} 
#     ${CMAKE_SOURCE_DIR}/globals.h
# )
# replace the strings between the line "add_executable(${PROJECT_NAME} " and the line ")" with the generated string
sed -i "/add_executable(\${PROJECT_NAME}/,/)/c\add_executable(\${PROJECT_NAME} \n${STR})" $CMAKELISTS_PATH

# done
echo "Done!"