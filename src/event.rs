use void::CVoid;
use base::Event;

#[repr(u32)]
pub enum EventType {
    Timer = 0x80000000,
    Runtime = 0x40000000,
    NotifyWait = 0x00000100,
    NotifySignal = 0x00000200,
    SignalExitBootServices = 0x00000201,
    SignalVirtualAddressChange = 0x60000202
}

#[repr(C)]
pub enum TimerDelay {
    Cancel = 0,
    Periodic = 1,
    Relative = 2
}

pub type EventNotify = extern "win64" fn(event: Event, context: *const CVoid);
