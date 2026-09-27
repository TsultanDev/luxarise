
module; // 1. Global Module Fragment

#include <print> // Include header biasa di dalam fragment
#include <GLFW/glfw3.h>
export module libr;

export GLFWwindow* create_window(){
    std::println("Window created");
    glfwInit();
    return glfwCreateWindow(800, 600, "My Window", nullptr, nullptr);
}
export void destroy_window(GLFWwindow* window){
    std::println("Destroy Window");
    glfwDestroyWindow(window);
    glfwTerminate();
}
export bool window_should_close(GLFWwindow* window){
    auto value = glfwWindowShouldClose(window);
    glfwPollEvents();
    return value;
}