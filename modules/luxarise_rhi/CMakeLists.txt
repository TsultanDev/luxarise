
find_package(Vulkan REQUIRED)

add_library(luxarise_rhi STATIC)

target_sources(luxarise_rhi PUBLIC
    FILE_SET CXX_MODULES
        BASE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/src"
        FILES 
        "${CMAKE_CURRENT_SOURCE_DIR}/src/lib.ccm"
        "${CMAKE_CURRENT_SOURCE_DIR}/src/instance.ccm"
        "${CMAKE_CURRENT_SOURCE_DIR}/src/adapter.ccm"
)

target_link_libraries(luxarise_rhi PUBLIC Vulkan::Vulkan)