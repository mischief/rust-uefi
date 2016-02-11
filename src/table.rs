use void::NotYetDef;
use guid::Guid;

#[repr(C)]
pub struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct RuntimeServicesInternal {
    header: TableHeader,
}

#[repr(C)]
pub struct ConfigurationTableInternal {
    vendor_guid: Guid,
    vendor_table: *const NotYetDef,
}

