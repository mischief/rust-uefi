//use core::ptr;
use core::mem;

use base::MemoryDescriptor;

use systemtable;

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
