cmake_minimum_required(VERSION 3.28)
project(luxarise LANGUAGES C)

# Menggunakan Standar C++23
set(CMAKE_C_STANDARD 23)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Wajib agar LSP (clangd / VS Code) bisa membaca path header & module
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Menambahkan executable project
add_executable(luxarise)

target_include_directories(luxarise
  PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include
  PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src
)
