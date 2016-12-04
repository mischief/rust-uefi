use core::mem;

use base::{Status, MemoryDescriptor};

use systemtable::*;

pub fn grow_buffer<T>(status: &mut Status, buffer: &mut *mut T, buffer_size: usize) -> bool {
    let bs = get_system_table().boot_services();

    if *buffer == 0 as *mut T && buffer_size != 0 {
        *status = Status::BufferTooSmall;
    }

    let mut try_again: bool = false;
    if *status == Status::BufferTooSmall {
        if *buffer == 0 as *mut T {
            bs.free_pool::<T>(*buffer);
        }

        *buffer = match unsafe{ bs.allocate_pool::<T>(buffer_size) } {
            Ok(val) => val,
            Err(status) => panic!("Error! {}", status.str())
        };

        if *buffer != 0 as *mut T {
            try_again = true;
        } else {
            *status = Status::OutOfResources;
            try_again = false;
        }
    }

    if !try_again && *status != Status::Success && *buffer != 0 as *mut T {
        bs.free_pool::<T>(*buffer);
        *buffer = 0 as *mut T;
    }

    return try_again;
}

pub fn lib_memory_map(no_entries: &mut usize, map_key: *mut usize, descriptor_size: *mut usize, descriptor_version: *mut u32) -> Result<&'static MemoryDescriptor, Status> {
    let bs = get_system_table().boot_services();
    let mut status: Status = Status::Success;
    let mut buffer: *mut MemoryDescriptor = 0 as *mut MemoryDescriptor;
    let mut buffer_size: usize = mem::size_of::<MemoryDescriptor>();

    loop {
        if grow_buffer(&mut status, &mut buffer, buffer_size) {
            bs.get_memory_map(&mut buffer_size, buffer, map_key, descriptor_size, descriptor_version);
        } else {
            break;
        }
    }
    *no_entries = buffer_size / unsafe{ *descriptor_size };
    let r = unsafe { mem::transmute::<*mut MemoryDescriptor, &'static MemoryDescriptor>(buffer) };
    Ok(r)
}
