#![allow(dead_code)]
#![no_std]

mod void;
mod base;
mod guid;
mod table;
mod systemtable;
mod bootservices;
mod runtimeservices;
mod console;


use void::{NotYetDef};


pub use base::{Handle, Event, Status};
pub use guid::*;

pub use systemtable::*;

pub use bootservices::BootServices;

pub use runtimeservices::{ResetType, RuntimeServices};

pub use console::{InputKey, SimpleTextOutput, SimpleTextInput, Console};

#[repr(C)]
pub struct LoadedImageProtocol {
    revision: u32,
    parent_handle: ::base::Handle,
    system_table: *const NotYetDef,
    device_handle: Handle,
    file_path: *const NotYetDef,
    __reserved: *const NotYetDef,
    load_options_size: u32,
    load_options: *const NotYetDef,
    image_base: usize,
    image_size: u64,
    // image_code_type,
    // image_data_type,
    // unload,
}

