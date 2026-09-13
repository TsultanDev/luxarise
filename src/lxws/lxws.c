#include <lxws/lxws.h>

#include <stdint.h>
#include <stdlib.h>
#include <Windows.h>

typedef struct LxwsWindow_S{
    HWND        hwnd       ;
    HINSTANCE   instance   ;
    BOOL        shouldClose;
} LxwsWindow_S;

static const char S_LXWS_WINDOW_CLASS_NAME[] = "LxwsWindowClass";
static int        S_lxwsWindowClassRegistered = 0;

static LRESULT CALLBACK LxwsWindowProc(HWND hwnd, UINT message, WPARAM wParam, LPARAM lParam){
    LxwsWindow_S* window = (LxwsWindow_S*)GetWindowLongPtrA(hwnd, GWLP_USERDATA);
    switch(message){
        case WM_CLOSE:
            if(window){
                window->shouldClose = TRUE;
            }
            return 0;
        case WM_DESTROY:
            if(window){
                window->shouldClose = TRUE;
            }
            return 0;
        default:
            break;
    }
    return DefWindowProcA(hwnd, message, wParam, lParam);
}

static BOOL LxwsRegisterWindowClass(HINSTANCE instance){
    if(S_lxwsWindowClassRegistered){
        return TRUE;
    }

    WNDCLASSEXA windowClass;
    ZeroMemory(&windowClass, sizeof(windowClass));
    windowClass.cbSize        = sizeof(WNDCLASSEXA);
    windowClass.style         = CS_HREDRAW | CS_VREDRAW;
    windowClass.lpfnWndProc   = LxwsWindowProc;
    windowClass.hInstance     = instance;
    windowClass.hIcon         = LoadIconA(NULL, IDI_APPLICATION);
    windowClass.hCursor       = LoadCursorA(NULL, IDC_ARROW);
    windowClass.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
    windowClass.lpszClassName = S_LXWS_WINDOW_CLASS_NAME;

    if(!RegisterClassExA(&windowClass)){
        return FALSE;
    }
    S_lxwsWindowClassRegistered = 1;
    return TRUE;
}

LxwsWindow lxws_create_window(LxwsWindowCreateInfo* pCreateInfo){
    if(!pCreateInfo){
        return NULL;
    }
    if(pCreateInfo->sType != LXWS_STRUCTURE_TYPE_WINDOW_CREATE_INFO){
        return NULL;
    }

    HINSTANCE instance = GetModuleHandleA(NULL);
    if(!LxwsRegisterWindowClass(instance)){
        return NULL;
    }

    DWORD style = WS_OVERLAPPEDWINDOW;
    if(!(pCreateInfo->flags & LXWS_WINDOW_CREATE_FLAG_RESIZABLE)){
        style &= ~(WS_THICKFRAME | WS_MAXIMIZEBOX);
    }

    uint32_t width  = pCreateInfo->width  > 0 ? pCreateInfo->width  : 800;
    uint32_t height = pCreateInfo->height > 0 ? pCreateInfo->height : 600;

    RECT rect = {0, 0, (LONG)width, (LONG)height};
    AdjustWindowRectEx(&rect, style, FALSE, 0);

    LxwsWindow_S* window = (LxwsWindow_S*)calloc(1, sizeof(LxwsWindow_S));
    if(!window){
        return NULL;
    }
    window->instance = instance;

    HWND hwnd = CreateWindowExA(
        0,
        S_LXWS_WINDOW_CLASS_NAME,
        pCreateInfo->pTitle ? pCreateInfo->pTitle : "Luxarise",
        style,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        rect.right  - rect.left,
        rect.bottom - rect.top,
        NULL,
        NULL,
        instance,
        NULL
    );
    if(!hwnd){
        free(window);
        return NULL;
    }

    window->hwnd = hwnd;
    SetWindowLongPtrA(hwnd, GWLP_USERDATA, (LONG_PTR)window);

    if(pCreateInfo->flags & LXWS_WINDOW_CREATE_FLAG_MAXIMIZED){
        ShowWindow(hwnd, SW_MAXIMIZE);
    }else{
        ShowWindow(hwnd, SW_SHOW);
    }

    return window;
}

void lxws_destroy_window(LxwsWindow window){
    if(!window){
        return;
    }
    if(window->hwnd){
        DestroyWindow(window->hwnd);
        window->hwnd = NULL;
    }
    free(window);
}

int lxws_poll_events(void){
    MSG message;
    int count = 0;
    while(PeekMessageA(&message, NULL, 0, 0, PM_REMOVE)){
        if(message.message == WM_QUIT){
            return -1;
        }
        TranslateMessage(&message);
        DispatchMessageA(&message);
        count++;
    }
    return count;
}

int lxws_window_close(LxwsWindow window){
    return (window && window->shouldClose) ? 1 : 0;
}