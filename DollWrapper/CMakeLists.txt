cmake_minimum_required(VERSION 3.15)

project(DollWrapper)

# specifies the source file directory
set(CMAKE_SOURCE_DIR src)

# specified the binary output directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# config C standard
set(CMAKE_C_STANDARD 23)

# config C extensions
set(CMAKE_C_EXTENSIONS ON)

# set debug mode
set(CMAKE_BUILD_TYPE Debug)

add_executable(${PROJECT_NAME} 
    ${CMAKE_SOURCE_DIR}/loop.c
    ${CMAKE_SOURCE_DIR}/main.c
    ${CMAKE_SOURCE_DIR}/popen2.c
    ${CMAKE_SOURCE_DIR}/setup.c
    ${CMAKE_SOURCE_DIR}/h/globals.h
    ${CMAKE_SOURCE_DIR}/h/loop.h
    ${CMAKE_SOURCE_DIR}/h/main.h
    ${CMAKE_SOURCE_DIR}/h/popen2.h
    ${CMAKE_SOURCE_DIR}/h/setup.h
    ${CMAKE_SOURCE_DIR}/h/types.h
    ${CMAKE_SOURCE_DIR}/h/vars.h
)

# specify output binary name
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME "doll")