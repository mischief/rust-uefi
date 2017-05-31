#[repr(usize)]
pub enum TPL {
    Application = 4,
    Callback = 8,
    Notify = 16,
    HighLevel = 31
}
