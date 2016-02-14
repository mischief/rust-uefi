use core::ptr;
use core::fmt;

use void::NotYetDef;
use base::Status;
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

/// UEFI Time structure.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Time {
    /// Year [1900 - 9999]
    pub year: u16,

    /// Month [1 - 12]
    pub month: u8,

    /// Day [1 - 31]
    pub day: u8,

    /// Hour [0 - 23]
    pub hour: u8,

    /// Minute [0 - 59]
    pub minute: u8,

    /// Second [0 - 59]
    pub second: u8,

    __pad1: u8,

    /// Nanosecond [0 - 999,999,999]
    pub nanosecond: u32,

    /// Timezone [-1440 - 1440] or 2047 for "unspecified timezone"
    pub timezone: i16,

    daylight: u8,
    __pad2: u8,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

#[repr(C)]
struct TimeCapabilities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: bool,
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
    set_virtual_address_map: *const NotYetDef,
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

