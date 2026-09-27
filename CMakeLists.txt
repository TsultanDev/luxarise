cmake_minimum_required(VERSION 3.28)

project(MyModuleApp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(SRC_DIR src)

find_package(glfw3 REQUIRED)

add_executable(luxarise)

target_sources(luxarise
    PRIVATE
        ${SRC_DIR}/main.cc
    PUBLIC
        FILE_SET CXX_MODULES FILES
            ${SRC_DIR}/libr.cppm
)

target_link_libraries(luxarise PUBLIC glfw)