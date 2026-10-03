set(INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/include")
set(SOURCES_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src")

add_executable(application "${SOURCES_DIR}/main.cc")

target_link_libraries(application PRIVATE luxarise)

