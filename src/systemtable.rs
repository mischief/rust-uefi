use base;
use table;
use bootservices;
use runtimeservices;
use console;

/// UEFI System Table.
/// http://wiki.phoenix.com/wiki/index.php/EFI_SYSTEM_TABLE
#[repr(C)]
pub struct SystemTable {
    header: table::TableHeader,
    vendor: *const u16,
    revision: u32,
    con_in_handle: base::Handle,
    con_in: &'static console::SimpleTextInputProtocol,
    con_out_handle: base::Handle,
    con_out: &'static console::SimpleTextOutputProtocol,
    std_err_handle: base::Handle,
    std_err: &'static console::SimpleTextOutputProtocol,
    runtime_services: &'static runtimeservices::RuntimeServices,
    boot_services: &'static bootservices::BootServices,
    configuration_table_entries: usize,
    configuration_table: &'static table::ConfigurationTableInternal,
}

impl SystemTable {
    pub fn console(&'static self) -> console::Console {
        console::Console::new(self, self.con_in, self.con_out)
    }

    pub fn boot_services(&self) -> &'static bootservices::BootServices {
        return self.boot_services;
    }

    pub fn runtime_services(&self) -> &'static runtimeservices::RuntimeServices {
        return self.runtime_services;
    }

    pub fn vendor(&self) -> *const u16 {
        return self.vendor
    }
}

static mut SYSTEM_TABLE : *const SystemTable = 0 as *const SystemTable;

/// Set System Table handle.
pub fn set_system_table(table: *const SystemTable) -> &'static SystemTable {
    unsafe {
        SYSTEM_TABLE = table;
    }

    get_system_table()
}

/// Retreive System Table handle.
pub fn get_system_table() -> &'static SystemTable {
    unsafe {
        return &*SYSTEM_TABLE;
    }
}

