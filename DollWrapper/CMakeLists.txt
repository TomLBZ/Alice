cmake_minimum_required(VERSION 3.15)

project(DollWrapper)

# specifies the source file directory
set(CMAKE_SOURCE_DIR src)

# specifies additional include directories
include_directories(${CMAKE_SOURCE_DIR}/h ${CMAKE_SOURCE_DIR}/externals)

# specified the binary output directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# config C standard
set(CMAKE_C_STANDARD 23)

# config C extensions
set(CMAKE_C_EXTENSIONS ON)

# config C++ standard
set(CMAKE_CXX_STANDARD 23)

# config C++ extensions
set(CMAKE_CXX_EXTENSIONS ON)

# config C++ flags
# -march=native -O3 -Wpedantic -Wall -Wextra -Wsign-conversion -Wconversion
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -march=native -O3 -Wall -Wextra -Wpedantic -Werror -Wsign-conversion -Wconversion -Wno-unused-parameter -Wno-unused-variable -Wno-unused-function -Wno-unused-but-set-variable -Wno-unused-but-set-parameter -Wno-unused-result -Wno-unused-label -Wno-unused-value -Wno-unused-local-typedefs -Wno-unused-macros -Wno-unused-const-variable -Wno-unused-const-variable")

# set debug mode
set(CMAKE_BUILD_TYPE Debug)

add_executable(${PROJECT_NAME} 
    ${CMAKE_SOURCE_DIR}/loop.cpp
    ${CMAKE_SOURCE_DIR}/main.cpp
    ${CMAKE_SOURCE_DIR}/popen2.cpp
    ${CMAKE_SOURCE_DIR}/setup.cpp
    # ${CMAKE_SOURCE_DIR}/h/globals.h
    # ${CMAKE_SOURCE_DIR}/h/loop.h
    # ${CMAKE_SOURCE_DIR}/h/main.h
    # ${CMAKE_SOURCE_DIR}/h/popen2.h
    # ${CMAKE_SOURCE_DIR}/h/setup.h
    # ${CMAKE_SOURCE_DIR}/h/types.h
    # ${CMAKE_SOURCE_DIR}/h/vars.h
)

# specify output binary name
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME "doll")