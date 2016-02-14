use core::slice;
use core::fmt;

use void::CVoid;
use systemtable;

/// Type for EFI_HANDLE.
#[repr(C)]
pub struct Handle(*mut CVoid);

pub struct Handles(*const Handle, usize);

impl Handles {
    pub fn new(p: *const Handle, len: usize) -> Handles {
        return Handles(p, len);
    }
}

#[cfg(target_os = "unknown")]
impl ::core::ops::Drop for Handles {
	fn drop(&mut self) {
        let bs = systemtable::get_system_table().boot_services();
        bs.free_pool(self.0);
    }
}

impl<'a> ::core::iter::IntoIterator for &'a Handles {
    type Item = &'a Handle;
    type IntoIter = HandlesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        HandlesIterator {
            handles: self,
            offset: 0,
        }
    }
}

pub struct HandlesIterator<'a> {
    handles: &'a Handles,
    offset: usize,
}

impl<'a> ::core::iter::Iterator for HandlesIterator<'a> {
    type Item = &'a Handle;

    fn next(&mut self) -> Option<Self::Item> {
        let sl = unsafe { slice::from_raw_parts(self.handles.0, self.handles.1) };
        let item = sl.get(self.offset);
        self.offset += 1;
        return item;
    }
}

impl<'a> ::core::iter::ExactSizeIterator for HandlesIterator<'a> {
    fn len(&self) -> usize {
        self.handles.1
    }
}

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
    VolumeCorrupted = 10,
    VolumeFull = 11,
    NoMedia = 12,
    MediaChanged = 13,
    NotFound = 14,
    AccessDenied = 15,
    NoResponse = 16,
    NoMapping = 17,
    Timeout = 18,
    NotStarted = 19,
    AlreadyStarted = 20,
    Aborted = 21,
    IcmpError = 22,
    TftpError = 23,
    ProtocolError = 24,
    IncompatibleVersion = 25,
    SecurityViolation = 26,
    CrcError = 27,
    EndOfMedia = 28,
    EndOfFile = 31,
}

impl Status {
    pub fn str(&self) -> &'static str {
        match *self {
            Status::Success => "success",
            Status::LoadError => "load error",
            Status::InvalidParameter => "invalid parameter",
            Status::Unsupported => "unsupported",
            Status::BadBufferSize => "bad buffer size",
            Status::BufferTooSmall => "buffer too small",
            Status::NotReady => "not ready",
            Status::DeviceError => "device error",
            Status::WriteProtected => "write protected",
            Status::OutOfResources => "out of resources",
            Status::VolumeCorrupted => "volume corrupted",
            Status::VolumeFull => "volume full",
            Status::NoMedia => "no media",
            Status::MediaChanged => "media changed",
            Status::NotFound => "not found",
            Status::AccessDenied => "access denied",
            Status::NoResponse => "no response",
            Status::NoMapping => "no mapping",
            Status::Timeout => "timeout",
            Status::NotStarted => "not started",
            Status::AlreadyStarted => "already started",
            Status::Aborted => "aborted",
            Status::IcmpError => "ICMP error",
            Status::TftpError => "TFTP error",
            Status::ProtocolError => "protocol error",
            Status::IncompatibleVersion => "incompatible version",
            Status::SecurityViolation => "security violation",
            Status::CrcError => "CRC error",
            Status::EndOfMedia => "end of media",
            Status::EndOfFile => "end of file",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

#[test]
fn status_str() {
    assert_eq!(Status::Success.str(), "success");
}

