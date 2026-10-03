set(INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/include")
set(SOURCES_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src")

add_library(luxarise STATIC "${SOURCES_DIR}/luxarise.cc")
target_include_directories(luxarise PUBLIC "${INCLUDE_DIR}")