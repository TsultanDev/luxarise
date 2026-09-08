#[cfg(target_os = "windows")]
use windows::{
    Win32::{
        Foundation::{HINSTANCE as RawHandle, HWND as RawWindow},
        Graphics::Gdi::{COLOR_WINDOW, HBRUSH, UpdateWindow},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, GetMessageW, IDC_ARROW,
            LoadCursorW, MSG, RegisterClassW, SW_SHOW, ShowWindow, WINDOW_EX_STYLE, WNDCLASSEXW,
            WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
    core::{PCWSTR, w},
};

#[cfg(target_os = "windows")]
pub struct Window {
    raw_handle: RawHandle,
    raw_window: RawWindow,
}
#[cfg(target_os = "windows")]
impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Window, Exception> {
        unsafe {
            let instance = match GetModuleHandleW(None) {
                Ok(h) => h,
                Err(_) => return Err(Exception::SYSTEM_ERROR),
            };

            let window_class = w!("RustWin32Class");

            // Mendefinisikan kelas jendela (Window Class)
            let wc = WNDCLASSW {
                hCursor: match LoadCursorW(None, IDC_ARROW) {
                    Ok(p) => p,
                    Err(_) => return Err(Exception::SYSTEM_ERROR),
                },
                hInstance: RawHandle(instance.0),
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: None,
                hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as *mut _),
                ..Default::default()
            };

            RegisterClassW(&wc);

            let w_title: Vec<u16> = title.encode_utf16().collect();
            w_title.push(0);
            let new_title = PCWSTR(w_title.as_ptr());

            let hwnd = match CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                window_class,
                new_title,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                None,
                None,
                Some(RawHandle(instance.0)),
                None,
            ) {
                Ok(w) => w,
                Err(_) => return Err(Exception::SYSTEM_ERROR),
            };

            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);

            // Message Loop (Pusat penanganan event sistem operasi)
            let mut message = MSG::default();
            while GetMessageW(&mut message, RawWindow(0), 0, 0).into() {
                TranslateMessage(&message);
                DispatchMessageA(&message);
            }
        }

        Ok(())
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Exception: u32 {
        const NULL_STRING = 0x00000001;
        const SIZE_ZERO_VALUE = 0x00000002;
        const SIZE_OUT_OF_THE_BOUNDS = 0x00000004;
        const SYSTEM_ERROR = 0x00000008;
    }
}
