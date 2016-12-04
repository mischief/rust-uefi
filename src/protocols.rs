use void;
use base;
use guid;
use systemtable;

pub trait Protocol {
    fn guid() -> &'static guid::Guid;
}

/// GUID for UEFI protocol for loaded images
pub static EFI_LOADED_IMAGE_PROTOCOL_GUID: guid::Guid = guid::Guid(0x5B1B31A1, 0x9562, 0x11d2, [0x8E,0x3F,0x00,0xA0,0xC9,0x69,0x72,0x3B]);

#[derive(Debug)]
#[repr(C)]
pub struct LoadedImageProtocol {
    revision: u32,
    parent_handle: base::Handle,
    system_table: *const void::NotYetDef,
    device_handle: base::Handle,
    file_path: *const void::NotYetDef,
    __reserved: *const void::NotYetDef,
    load_options_size: u32,
    load_options: *const void::NotYetDef,
    pub image_base: usize,
    pub image_size: u64,
    image_code_type: base::MemoryType,
    pub image_data_type: base::MemoryType,

    //unload: unsafe extern "win64" fn(handle: Handle),
    unload: *const void::NotYetDef,
}

impl Protocol for LoadedImageProtocol {
    fn guid() -> &'static guid::Guid {
        return &EFI_LOADED_IMAGE_PROTOCOL_GUID;
    }
}

