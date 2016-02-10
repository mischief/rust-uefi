/// Type for EFI_GUID.
#[repr(C)]
pub struct Guid(u32, u16, u16, [u8; 8]);

/// GUID for UEFI protocol for loaded images
pub static EFI_LOADED_IMAGE_PROTOCOL_GUID: Guid = Guid(0x5B1B31A1, 0x9562, 0x11d2, [0x8E,0x3F,0x00,0xA0,0xC9,0x69,0x72,0x3B]);

