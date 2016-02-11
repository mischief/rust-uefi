use void::{NotYetDef, CVoid};
use base;
use guid;
use table;
use systemtable;

/// See http://wiki.phoenix.com/wiki/index.php/EFI_BOOT_SERVICES
#[repr(C)]
pub struct BootServicesInternal {
    header: table::TableHeader,
    raise_tpl: *const NotYetDef,
    restore_tpl: *const NotYetDef,
    allocate_pages: *const NotYetDef,
    free_pages: *const NotYetDef,
    get_memory_map: *const NotYetDef,
    allocate_pool: *const NotYetDef,
    free_pool: *const NotYetDef,
    create_event: *const NotYetDef,
    set_timer: *const NotYetDef,
    // typedef EFI_STATUS (EFIAPI *EFI_WAIT_FOR_EVENT) (IN UINTN NumberOfEvents, IN EFI_EVENT *Event, OUT UINTN *Index);
    wait_for_event: unsafe extern "win64" fn(usize, *const base::Event, *mut usize) -> base::Status,
    signal_event: *const NotYetDef,
    close_event: *const NotYetDef,
    check_event: *const NotYetDef,
    install_protocol_interface: *const NotYetDef,
    reinstall_protocol_interface: *const NotYetDef,
    uninstall_protocol_interface: *const NotYetDef,
    handle_protocol: unsafe extern "win64" fn(base::Handle, &guid::Guid, &mut *mut CVoid) -> base::Status,
    __reserved: *const NotYetDef,
    register_protocol_notify: *const NotYetDef,
    locate_handle: *const NotYetDef,
    locate_device_path: *const NotYetDef,
    install_configuration_table: *const NotYetDef,
    load_image: *const NotYetDef,
    start_image: *const NotYetDef,
    exit: *const NotYetDef,
    unload_image: *const NotYetDef,
    exit_boot_services: *const NotYetDef,
    get_next_monotonic_count: *const NotYetDef,
    stall: unsafe extern "win64" fn(usize) -> base::Status,
    set_watchdog_timer: *const NotYetDef,
    connect_controller: *const NotYetDef,
    disconnect_controller: *const NotYetDef,
    open_protocol: *const NotYetDef,
    close_protocol: *const NotYetDef,
    open_protocol_information: *const NotYetDef,
    protocols_per_handle: *const NotYetDef,
    locate_handle_buffer: *const NotYetDef,
    locate_protocol: *const NotYetDef,
    install_multiple_protocol_interfaces: *const NotYetDef,
    uninstall_multiple_protocol_interfaces: *const NotYetDef,
    calculate_crc32: *const NotYetDef,
    copy_mem: unsafe extern "win64" fn(*mut CVoid, *mut CVoid, usize),
    set_mem: unsafe extern "win64" fn(*mut CVoid, usize, u8),
    create_event_ex: *const NotYetDef,
}

#[repr(C)]
pub struct BootServices(&'static BootServicesInternal);

impl BootServices {
    pub fn wait_for_event(&self, events: &[base::Event]) -> Result<usize, base::Status> {
        // XXX: asserts sizeof *Cvoid == sizeof Event
        if false {
            use core::mem;
            unsafe { mem::transmute::<*const CVoid, base::Event>(0x1 as *const CVoid); }
        }

        let mut index : usize = 0;

        let result = unsafe { (self.0.wait_for_event)(events.len(), events.as_ptr(), &mut index) };
        if result != base::Status::Success {
            return Err(result);
        }

        Ok(index)
    }

    // fix me
    pub fn handle_protocol(&self, handle: base::Handle, uuid: &guid::Guid) -> *const super::LoadedImageProtocol {
        let mut ptr : *mut CVoid = 0 as *mut CVoid;

        let bs = self.0;

        unsafe {
            let status = (bs.handle_protocol)(handle, uuid, &mut ptr);
            if status != base::Status::Success {
                use console::SimpleTextOutput;
                let st = systemtable::get_system_table();
                st.console().write("bs> bad res from handle_protocol!\n");
            }
        }

        return ptr as *const super::LoadedImageProtocol;
    }

    /// Sleep for a number of microseconds.
    pub fn stall(&self, microseconds: usize) {
        unsafe {
            (self.0.stall)(microseconds);
        }
    }

    pub fn copy_mem(&self, dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        unsafe {
            (self.0.copy_mem)(dest as *mut CVoid, src as *mut CVoid, n);
        }

        return dest;
    }

    pub fn set_mem(&self, s: *mut u8, c: u8, n: usize) -> *mut u8 {
        unsafe {
            (self.0.set_mem)(s as *mut CVoid, n, c);
        }

        return s;
    }
}

