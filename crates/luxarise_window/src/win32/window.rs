use std::{default, ptr};

use windows::{Win32::{Foundation::HWND, UI::WindowsAndMessaging::{CS_HREDRAW as StyleHorizaontalDraw, CS_VREDRAW as StyleVerticalDraw, HICON, WNDCLASSA}}, core::PCSTR, };

use crate::win32::handle::{AnsiString, Handle};

pub enum Exception {
    NullString,
    SizeOfZero,
    SizeOutOfTheBounds,
    UnvalidHandleCode,
}

pub struct Window(HWND);
pub struct WindowCreateInfo<'a> {
    title: &'a str,
    width: u32,
    height: u32,
}
impl Handle {
    pub fn create_window(&self, create_info: &WindowCreateInfo) -> Result<Window, Exception> {
        match self.get_handle_code() {
            1 => {
                const CLASS_NAME = "Luxarise";

                let class_name = "MyWindowClass\0".as_ptr();

                let wnd_class = WNDCLASSA {
                    style: StyleHorizaontalDraw | StyleVerticalDraw,
                    lpfnWndProc: None,
                    cbClsExtra: 0,
                    cbWndExtra: 0,
                    hInstance: self.get_handle_instance(),
                    lpszMenuName: PCSTR(ptr::null()),
                    lpszClassName: PCSTR(class_name) as AnsiString,
                    ..unsafe { std::mem::zeroed() }
                };

                Ok(())

            }
            2 => {

            }
            _ => return Err(Exception::UnvalidHandleCode),
        }
    }
}
