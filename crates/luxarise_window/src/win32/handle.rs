use std::ptr;

use windows::{
    Win32::{
        Foundation::HINSTANCE,
        System::LibraryLoader::{GetModuleHandleA, GetModuleHandleW},
    },
    core::{PCSTR, PCWSTR},
};

pub type AnsiString = PCSTR;
pub type WideString = PCWSTR;

use crate::win32::handle::Exception::WrongHandleType;

#[derive(Debug)]
pub enum Exception {
    WrongHandleType,
    FailedToLoad,
}

pub struct Handle(HINSTANCE, u32);
impl Handle {
    pub fn load_handle(create_info: &HandleCreateInfo) -> Result<Handle, Exception> {
        unsafe {
            match create_info.enum_type {
                HandleTypes::AnsiLegacy(pointer) => {
                    let instance = GetModuleHandleA(pointer);
                    match instance {
                        Ok(h) => return Ok(Handle(HINSTANCE(h.0), 1)),
                        Err(_) => return Err(Exception::FailedToLoad),
                    }
                }
                HandleTypes::WideLegacy(pointer) => {
                    let instance = GetModuleHandleW(pointer);
                    match instance {
                        Ok(h) => return Ok(Handle(HINSTANCE(h.0), 2)),
                        Err(_) => return Err(Exception::FailedToLoad),
                    }
                }
                _ => return Err(WrongHandleType),
            }
        }
    }
    pub fn get_handle_code(&self) -> u32 {
        self.1
    }
    pub fn get_handle_instance(&self) -> HINSTANCE {
        self.0
    }
}

bitflags::bitflags! {
    pub struct HandleCreateFlags : u32{
        const NONE = 0x00000000;
        const MAX_FLAGS = 0x11111111;
    }
}

pub enum ExtendedHandleCreateInfo {}
pub enum HandleTypes {
    AnsiLegacy(AnsiString),
    WideLegacy(WideString),
}
pub struct HandleCreateInfo {
    pub flags: HandleCreateFlags,
    pub enum_type: HandleTypes,
    pub next: Vec<ExtendedHandleCreateInfo>,
}
impl Default for HandleCreateInfo {
    fn default() -> Self {
        Self {
            enum_type: HandleTypes::AnsiLegacy(PCSTR(ptr::null()) as AnsiString),
            next: Vec::new(),
            flags: HandleCreateFlags::NONE,
        }
    }
}
impl HandleCreateInfo {
    pub fn set_enum_type(mut self, enum_type: HandleTypes) -> Self {
        self.enum_type = enum_type;
        self
    }
    pub fn set_next(mut self, next_info: ExtendedHandleCreateInfo) -> Self {
        self.next.push(next_info);
        self
    }
    pub fn set_flags(mut self, flags: HandleCreateFlags) -> Self {
        self.flags = flags;
        self
    }
}
