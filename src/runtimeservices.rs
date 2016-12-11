use core::ptr;

use void::NotYetDef;
use base::{Status, Time, TimeCapabilities, MemoryDescriptor};
use guid::Guid;
use table::TableHeader;

/// Reset type passed to RuntimeServices.reset_system
#[repr(C)]
pub enum ResetType {
    Cold = 0,
    Warm = 1,
    Shutdown = 2,
    PlatformSpecific = 3,
}

/// UEFI Runtime Services.
/// http://wiki.phoenix.com/wiki/index.php/EFI_RUNTIME_SERVICES
#[repr(C)]
pub struct RuntimeServices {
    header: TableHeader,
    get_time: unsafe extern "win64" fn(time: &mut Time, capabilities: *mut TimeCapabilities) -> Status,
    set_time: *const NotYetDef,
    get_wakeup_time: *const NotYetDef,
    set_wakeup_time: *const NotYetDef,
    set_virtual_address_map: unsafe extern "win64" fn(memory_map_size: usize, descriptor_size: usize, descriptor_version: u32, efi_memory_descriptor: *const MemoryDescriptor) -> Status,
    convert_pointer: *const NotYetDef,
    get_variable: unsafe extern "win64" fn(name: *const u16, guid: &Guid, attributes: *mut u32, size: *mut usize, data: *mut u8) -> Status,
    get_next_variable_name: *const NotYetDef,
    set_variable: unsafe extern "win64" fn(name: *const u16, guid: &Guid, attributes: *const u32, size: *const usize, data: *const u8) -> Status,
    get_next_highest_monotonic_count: unsafe extern "win64" fn(count: *mut u32) -> Status,
    reset_system: unsafe extern "win64" fn(resettype: ResetType, status: Status, datasize: usize, data: *const u8) -> !,
    update_capsule: *const NotYetDef,
    query_capsule_capabilities: *const NotYetDef,
    query_variable_info: *const NotYetDef,
}

impl RuntimeServices {
    pub fn set_virtual_address_map(&self, memory_map_size: &usize, descriptor_size: &usize, descriptor_version: &u32, efi_memory_descriptor: *const MemoryDescriptor) -> Status {
        unsafe {
            (self.set_virtual_address_map)(*memory_map_size, *descriptor_size, *descriptor_version, efi_memory_descriptor)
        }
    }
    pub fn get_time(&self) -> Result<Time, Status> {
        let mut t : Time = Time::default();
        let status = unsafe { (self.get_time)(&mut t, ptr::null_mut()) };
        if status != Status::Success {
            return Err(status)
        }

        Ok(t)
    }

    pub fn reset_system(&self, reset_type: ResetType, status: Status) -> ! {
        unsafe {
            (self.reset_system)(reset_type, status, 0, ptr::null());
        }
    }
}

