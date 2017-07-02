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

pub use base::{Handle, Handles, Event, MemoryType, MemoryDescriptor, Status, Time};
pub use guid::*;

pub use systemtable::*;

pub use bootservices::BootServices;

pub use protocols::*;

pub use runtimeservices::{ResetType, RuntimeServices};

pub use console::{Attribute, ForegroundColor, BackgroundColor, InputKey, SimpleTextOutput, SimpleTextInput, Console};

use core::mem;

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

// return (memory_map, memory_map_size, map_key, descriptor_size, descriptor_version)
pub fn lib_memory_map() -> (&'static MemoryDescriptor,  usize, usize, usize, u32) {
    let bs = systemtable::get_system_table().boot_services();
    let mut buffer_size: usize = mem::size_of::<MemoryDescriptor>();

    loop {
        match unsafe { bs.get_memory_map(&mut buffer_size) } {
            Ok(val) => return val,
            Err(_) => { continue; },
        };
    }
}
