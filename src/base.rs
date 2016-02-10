use void::CVoid;

/// Type for EFI_HANDLE.
#[repr(C)]
pub struct Handle(*mut CVoid);

/// Type for EFI_EVENT.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Event(*mut CVoid);

/// Type for EFI_STATUS
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[derive(PartialEq, PartialOrd)]
pub enum Status {
    Success = 0,
    LoadError = 1,
    InvalidParameter = 2,
    Unsupported = 3,
    BadBufferSize = 4,
    BufferTooSmall = 5,
    NotReady = 6,
    DeviceError = 7,
    WriteProtected = 8,
    OutOfResources = 9,
}

