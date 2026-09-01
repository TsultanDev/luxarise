use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{BeginPaint, COLOR_WINDOW, EndPaint, HBRUSH, PAINTSTRUCT};
use windows::Win32::UI::WindowsAndMessaging::{
    CW_USEDEFAULT, DefWindowProcW, IDC_ARROW, LoadCursorW, PostQuitMessage, WINDOW_EX_STYLE,
    WM_DESTROY, WM_PAINT, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};
use windows::Win32::{
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{CreateWindowExW, RegisterClassW, WNDCLASSW},
};
use windows::core::{s, w};

pub struct Window {
    handle: HINSTANCE,
    window: HWND,
}

impl Window {
    pub fn create_window(width: u32, height: u32, title: &str) -> Window {
        unsafe {
            let instance = GetModuleHandleW(None)
                .expect("luxarise/window/win32 : Failed to get module handle!!");

            let class_name = w!("LuxariseWindow");

            let wnd_class = WNDCLASSW {
                hInstance: instance.into(),
                lpszClassName: class_name,
                lpfnWndProc: Some(wnd_proc),
                hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as _),
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                ..Default::default()
            };

            RegisterClassW(&wnd_class);

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                w!("Aplikasi Win32 Rust"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                None,
                None,
                Some(HINSTANCE(instance.0)),
                None,
            )
            .expect("luxarise/window/win32 : Failed to get module handle!!");

            Window {
                handle: HINSTANCE(instance.0),
                window: hwnd,
            }
        }
    }
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let _hdc = BeginPaint(hwnd, &mut ps);
                _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
