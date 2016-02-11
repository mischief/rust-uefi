use void::*;
use base::{Event, Status};
use systemtable;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InputKey {
    scan_code: u16,
    unicode_char: u16,
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
    test_string: *const NotYetDef,
    query_mode: *const NotYetDef,
    set_mode: *const NotYetDef,
    set_attribute: *const NotYetDef,
    clear_screen: *const NotYetDef,
    set_cursor_position: *const NotYetDef,
    enable_cursor: *const NotYetDef,
    mode: *const NotYetDef,
}

pub trait SimpleTextOutput {
    fn write_raw(&self, str: *const u16) -> Status;

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
}

impl SimpleTextInput for Console {
    fn read_key(&self) -> Result<InputKey, Status> {
        return Err(Status::NotReady);

        let bs = self.system_table.boot_services();

        let input = self.input;
        let keyevt = input.wait_for_key;
        let events : [Event; 1] = [keyevt];

        // returned key code
        let mut key = InputKey{scan_code: 0, unicode_char: 0};

        loop {
            // wait for key event
            let _ = bs.wait_for_event(&events);
            // get key
            let status = unsafe { (input.read_key_stroke)(input, &mut key) };
            match status {
                Status::Success => return Ok(key),
                Status::NotReady => continue,
                _ => return Err(status),
            }
        }
    }
}

