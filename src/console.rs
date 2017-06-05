use void::*;
use base::{Event, Status};
use systemtable;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InputKey {
    scan_code: u16,
    unicode_char: u16,
}

#[repr(u8)]
pub enum ForegroundColor {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[repr(u8)]
pub enum BackgroundColor {
    Black = 0x00,
    Blue = 0x10,
    Green = 0x20,
    Cyan = 0x30,
    Red = 0x40,
    Magenta = 0x50,
    Brown = 0x60,
    LightGray = 0x70,
}

#[derive(Clone, Copy, Debug)]
pub struct Attribute {
    fg: u8,
    bg: u8,
}

impl Attribute {
    pub fn new(fg: ForegroundColor, bg: BackgroundColor) -> Attribute {
        Attribute { fg: fg as u8, bg: bg as u8, }
    }

    fn to_efi_attribute(&self) -> usize {
        ((self.bg as usize) << 4) | self.fg as usize
    }
}

#[repr(C)]
pub struct SimpleTextInputProtocol {
    reset: unsafe extern "win64" fn(*const SimpleTextInputProtocol, u8) -> Status,
    read_key_stroke: unsafe extern "win64" fn(*const SimpleTextInputProtocol, &mut InputKey) -> Status,
    wait_for_key: Event,
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    reset: unsafe extern "win64" fn(*const SimpleTextOutputProtocol, u8) -> Status,
    output_string: unsafe extern "win64" fn(*const SimpleTextOutputProtocol, *const u16) -> Status,
    test_string: unsafe extern "win64" fn(*const SimpleTextOutputProtocol, *const u16) -> Status,
    query_mode: *const NotYetDef,
    set_mode: *const NotYetDef,
    set_attribute: unsafe extern "win64" fn(*const SimpleTextOutputProtocol, usize) -> Status,
    clear_screen: unsafe extern "win64" fn(*const SimpleTextOutputProtocol) -> Status,
    set_cursor_position: *const NotYetDef,
    enable_cursor: *const NotYetDef,
    mode: *const NotYetDef,
}

pub trait SimpleTextOutput {
    fn write_raw(&self, str: *const u16) -> Status;

    fn set_attribute(&self, attribute: Attribute) -> Status;

    fn write(&self, s: &str) -> Status {
        let mut buf = [0u16; 64];
        let mut i = 0;

        // don't write nothing
        if s.is_empty() {
            return Status::Success;
        }

        for c in s.chars() {
            buf[i] = c as u16;
            i += 1;

            // if we hit the end of buf, send output
            if i > buf.len() - 1 {
                buf[i] = 0;
                let status = self.write_raw(buf.as_ptr());
                if status != Status::Success {
                    return status
                }
                i = 0;
            }
        }

        // send the last bits
        if i > 0 {
            buf[i] = 0;
            let status = self.write_raw(buf.as_ptr());
            if status != Status::Success {
                return status
            }
        }

        return Status::Success
    }
}

pub trait SimpleTextInput {
    fn read_key_async(&self) -> Result<InputKey, Status>;
    fn read_key(&self) -> Result<InputKey, Status>;
}

pub struct Console {
    system_table: &'static systemtable::SystemTable,
    input: &'static SimpleTextInputProtocol,
    output: &'static SimpleTextOutputProtocol,
}

/// Console is a thin wrapper around UEFI SimpleTextInputProtocol and SimpleTextOutputProtocol.
impl Console {
    pub fn new(st: &'static systemtable::SystemTable, input: &'static SimpleTextInputProtocol, out: &'static SimpleTextOutputProtocol) -> Console {
        Console {
            system_table: st,
            input: input,
            output: out,
        }
    }

    pub fn reset(&self) -> Status {
        unsafe {
            (self.output.reset)(self.output, 1);
            (self.input.reset)(self.input, 1);
        }

        Status::Success
    }
}

impl SimpleTextOutput for Console {
    fn write_raw(&self, str: *const u16) -> Status {
        let output = self.output;
        let status = unsafe { (output.output_string)(output, str) };
        status
    }

    fn set_attribute(&self, attribute: Attribute) -> Status {
        unsafe {
            return (self.output.set_attribute)(self.output, attribute.to_efi_attribute());
        }
    }
}

impl SimpleTextInput for Console {
    fn read_key_async(&self) -> Result<InputKey, Status> {
        // returned key code
        let mut key = InputKey{scan_code: 0, unicode_char: 0};

        let status = unsafe { (self.input.read_key_stroke)(self.input, &mut key) };
        if status != Status::Success {
            return Err(status);
        }

        Ok(key)
    }

    fn read_key(&self) -> Result<InputKey, Status> {
        let bs = self.system_table.boot_services();
        let events : [Event; 1] = [self.input.wait_for_key];

        loop {
            // wait for key event
            let _ = bs.wait_for_event(&events);
            // get key
            let kr = self.read_key_async();

            match kr {
                Ok(k) => return Ok(k),
                Err(s) => {
                    match s {
                        // should not happen since we wait_for_event, but try again anyway.
                        Status::NotReady => continue,
                        _ => return Err(s),
                    }
                },
            }
        }
    }
}

