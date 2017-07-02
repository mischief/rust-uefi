#![allow(dead_code)]
#![no_std]

mod void;
mod base;
mod guid;
mod table;
mod systemtable;
mod bootservices;
mod runtimeservices;
mod protocols;
mod console;

pub use base::{Handle, Handles, Event, MemoryType, Status, Time};
pub use guid::*;

pub use systemtable::*;

pub use bootservices::BootServices;

pub use protocols::*;

pub use runtimeservices::{ResetType, RuntimeServices};

pub use console::{Attribute, ForegroundColor, BackgroundColor, InputKey, SimpleTextOutput, SimpleTextInput, Console};

static mut POOL_ALLOCATION_TYPE: base::MemoryType = base::MemoryType::BootServicesData;

pub fn initialize_lib(hdl: &base::Handle, sys: &systemtable::SystemTable) {
    let bs = systemtable::set_system_table(sys).boot_services();
    let loaded_image = match bs.handle_protocol::<LoadedImageProtocol>(hdl) {
        Ok(val) => val,
        Err(status) => panic!("Error! {}", status.str())
    };

    unsafe {
        POOL_ALLOCATION_TYPE = loaded_image.image_data_type.clone()
    }
}

pub fn get_pool_allocation_type() -> base::MemoryType {
    unsafe{ POOL_ALLOCATION_TYPE.clone() }
}
