add_library(luxarise STATIC)

target_sources(luxarise
    PUBLIC
        FILE_SET CXX_MODULES
        BASE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/src"
        FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/lib.ccm"
)

target_link_libraries(luxarise PRIVATE luxarise_window)