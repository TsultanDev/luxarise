# Library statis LXWS
add_library(LXWS STATIC
  src/lxws/lxws.c
)

target_include_directories(LXWS
  PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include
  PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src
)

target_link_libraries(LXWS PUBLIC user32)
