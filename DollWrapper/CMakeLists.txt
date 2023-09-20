cmake_minimum_required(VERSION 3.15)

project(DollWrapper)

# specifies the source file directory
set(CMAKE_SOURCE_DIR src)

# specified the binary output directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

add_executable(${PROJECT_NAME} 
    ${CMAKE_SOURCE_DIR}/globals.h
    ${CMAKE_SOURCE_DIR}/types.h
    ${CMAKE_SOURCE_DIR}/main.h
    ${CMAKE_SOURCE_DIR}/main.c
    ${CMAKE_SOURCE_DIR}/setup.h
    ${CMAKE_SOURCE_DIR}/setup.c
    ${CMAKE_SOURCE_DIR}/loop.h
    ${CMAKE_SOURCE_DIR}/loop.c
    ${CMAKE_SOURCE_DIR}/popen2.h
    ${CMAKE_SOURCE_DIR}/popen2.c
)

# specify output binary name
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME "doll")