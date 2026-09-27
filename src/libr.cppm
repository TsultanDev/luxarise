
module; // 1. Global Module Fragment

#include <print> // Include header biasa di dalam fragment

export module libr;

void private_func(){
    std::println("Hello ");
}
export void public_func(){
    std::println("Export");
    private_func();
}