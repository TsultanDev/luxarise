add_library(luxarise_window STATIC)

target_sources(luxarise_window
    PUBLIC
        FILE_SET CXX_MODULES
        BASE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/src"
        FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/lib.ccm"
        "${CMAKE_CURRENT_SOURCE_DIR}/src/window.ccm"
)