use std::ptr;

use luxarise::window::win32::handle::{Handle, HandleCreateInfo, HandleTypes};

fn main() {
    let mut handle_info = HandleCreateInfo::default();
    handle_info.enum_type = Some(HandleTypes::AnsiLegacy(ptr::null()));

    let handle = Handle::load_handle(&handle_info).expect("Failed");
    println!("App Exit Successfully");
}
