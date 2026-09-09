use std::{default, ptr};

use windows::{
    Win32::{
        Foundation::HINSTANCE,
        System::LibraryLoader::{GetModuleHandleA, GetModuleHandleExA, GetModuleHandleW},
    },
    core::{PCSTR as AnsiString, PCWSTR as WideString},
};

use crate::win32::handle::{Exception::WrongHandleType, HandleTypes::AnsiLegacy};

#[derive(Debug)]
pub enum Exception {
    WrongHandleType,
    FailedToLoad,
}

pub struct Handle(HINSTANCE);
impl Handle {
    pub fn load_handle(create_info: &HandleCreateInfo) -> Result<Handle, Exception> {
        unsafe {
            match create_info.enum_type {
                Some(HandleTypes::AnsiLegacy(pointer)) => {
                    let instance = GetModuleHandleA(AnsiString(pointer));
                    match instance {
                        Ok(h) => return Ok(Handle(HINSTANCE(h.0))),
                        Err(_) => return Err(Exception::FailedToLoad),
                    }
                }
                Some(HandleTypes::WideLegacy(pointer)) => {
                    let instance = GetModuleHandleW(WideString(pointer));
                    match instance {
                        Ok(h) => return Ok(Handle(HINSTANCE(h.0))),
                        Err(_) => return Err(Exception::FailedToLoad),
                    }
                }
                _ => return Err(WrongHandleType),
            }
        }
    }
}

pub enum ExtendedHandleCreateInfo {}
pub enum HandleTypes {
    AnsiLegacy(*const u8),
    WideLegacy(*const u16),
}
#[derive(Default)]
pub struct HandleCreateInfo {
    pub enum_type: Option<HandleTypes>,
    pub next: Vec<ExtendedHandleCreateInfo>,
}
