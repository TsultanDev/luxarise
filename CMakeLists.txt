cmake_minimum_required(VERSION 3.28)
project(luxarise LANGUAGES CXX)

# Menggunakan Standar C++23
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Wajib agar LSP (clangd / VS Code) bisa membaca path header & module
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Menambahkan executable project
add_executable(luxarise)

# Konfigurasi C++23 Module Interface (.ixx / .cppm)
target_sources(luxarise
    PRIVATE
        src/main.cc
    PUBLIC
        FILE_SET CXX_MODULES FILES
            src/application.ixx
)


