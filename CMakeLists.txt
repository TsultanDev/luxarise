cmake_minimum_required(VERSION 3.28)
project(luxarise CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_subdirectory("modules/luxarise")
add_subdirectory("modules/application")