#![allow(dead_code)]
#![no_std]

#[repr(C)]
pub struct Handle(*mut usize);

#[repr(C)]
struct Status(pub u32);

#[repr(C)]
struct Uuid(u32, u16, u16, [u8; 4]);

#[repr(C)]
struct InputKey {
    scan_code: u16,
    unicode_char: u16,
}

#[repr(C)]
struct SimpleTextInputProtocol {
    reset: *const usize,
    read_key_stroke: *const usize,
    waitforkey: *const usize,
}

type TextReset = extern "win64" fn(*const SimpleTextOutputProtocol, u8) -> Status;
type TextOutputString = extern "win64" fn(*const SimpleTextOutputProtocol, *const u16) -> Status;

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    reset: TextReset,
    output_string: TextOutputString,
    test_string: *const usize,
    query_mode: *const usize,
    set_mode: *const usize,
    set_attribute: *const usize,
    clear_screen: *const usize,
    set_cursor_position: *const usize,
    enable_cursor: *const usize,
    mode: *const usize,
}

#[repr(C)]
struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    header: TableHeader,
    vendor: *const u16,
    revision: u32,
    con_in_handle: Handle,
    con_in: *const SimpleTextInputProtocol,
    con_out_handle: Handle,
    con_out: *const SimpleTextOutputProtocol,
}

impl SystemTable {
    pub fn console(&self) -> Console {
        Console {
            input: self.con_in,
            output: self.con_out,
        }
    }
}

/// TODO: store internal copy here?
static mut SYSTEM_TABLE : *const SystemTable = 0 as *const SystemTable;

pub trait SimpleTextOutput {
    unsafe fn write_raw(&self, str: *const u16);

    /// # Examples
    ///
    /// ```ignore
    /// #[start]
    /// #[no_mangle]
    /// pub extern "win64" fn efimain(_: uefi::Handle, st: &'static uefi::SystemTable) -> uefi::Status {
    ///     st.console().write("Hello, world!\r\n");
    ///
    ///     let status = uefi::Status(0);
    ///     return status;
    /// }
    /// ```
    fn write(&self, str: &str) {
        let mut buf = [0u16; 64];
        let mut i = 0;

        if str.is_empty() {
            return
        }

        for c in str.chars() {
            buf[i] = c as u16;
            i += 1;
            if i > buf.len() - 1 {
                buf[i] = 0;
                unsafe {
                    self.write_raw(buf.as_ptr());
                }
            }
        }

        buf[i] = 0;
        unsafe {
            self.write_raw(buf.as_ptr());
        }
    }
}

pub struct Console {
    input: *const SimpleTextInputProtocol,
    output: *const SimpleTextOutputProtocol,
}

impl SimpleTextOutput for Console {
    unsafe fn write_raw(&self, str: *const u16) {
        ((*(*self).output).output_string)(self.output, str);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

