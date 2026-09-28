cmake_minimum_required(VERSION 3.28)

# 2. Inisialisasi Proyek
project(MyModuleApp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# 3. Panggil Subdirectory
add_subdirectory(modules/luxarise_window)
add_subdirectory(modules/luxarise_rhi)
add_subdirectory(modules/luxarise)
add_subdirectory(modules/application)