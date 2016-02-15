use core::fmt;

/// Type for EFI_GUID.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}", self.0,
               self.1,
               self.2,
               self.3[0], self.3[1],
               self.3[2], self.3[3], self.3[4], self.3[5], self.3[6], self.3[7])
    }
}

