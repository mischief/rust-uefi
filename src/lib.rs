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

