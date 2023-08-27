cmake_minimum_required(VERSION 3.15)

project(Echo)

# specifies the source file directory
set(CMAKE_SOURCE_DIR src)

# specified the binary output directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

add_executable(${PROJECT_NAME} ${CMAKE_SOURCE_DIR}/echo.c)