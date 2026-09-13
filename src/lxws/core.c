#include "core.h"
#include <stdint.h>

typedef struct LxwsWindow_S{
    uint32_t id;
    const char* pTitle;
} LxwsWindow_S;

LxwsWindow lxws_create_window(LxwsWindowCreateInfo* pCreateInfo){

}
void lxws_destroy_window(LxwsWindow window){

}
int lxws_window_close(LxwsWindow){

}
