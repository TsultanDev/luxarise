
#include <stdint.h>

typedef enum LxwsStructureType{
    LXWS_STRUCTURE_TYPE_WINDOW_CREATE_INFO,
}LxwsStructureType;

typedef enum LxwsWindowCreateFlags{
    LXWS_WINDOW_CREATE_FLAG_NONE        = 0x00000000,
    LXWS_WINDOW_CREATE_FLAG_MAXIMIZED   = 0x00000001,
    LXWS_WINDOW_CREATE_FLAG_RESIZABLE   = 0x00000002,
}LxwsWindowCreateFlags;

typedef struct LxwsWindowCreateInfo{
    LxwsStructureType   sType   ;
    const char*         pTitle  ;
    uint32_t            width   ;
    uint32_t            height  ;
    void*               pNext   ;
} LxwsWindowCreateInfo;

typedef struct LxwsWindow_S* LxwsWindow;

LxwsWindow lxws_create_window(LxwsWindowCreateInfo* pCreateInfo);
void lxws_destroy_window(LxwsWindow window);
int lxws_poll_events();
