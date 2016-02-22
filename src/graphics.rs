use core::ptr;

use base::Status;
use guid::Guid;

pub static EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: Guid = Guid(0x9042a9de, 0x23dc, 0x4a38, [0x96,0xfb,0x7a,0xde,0xd0,0x80,0x51,0x6a]);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point { Point{x: x, y: y } }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { red: r, green: g, blue: b, reserved: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum PixelFormat {
    /// PixelRedGreenBlueReserved8BitPerColor
    RedGreenBlue = 0,
    /// PixelBlueGreenRedReserved8BitPerColor
    BlueGreenRed = 1,
    /// PixelBitMask
    BitMask = 2,
    /// PixelBltOnly
    BltOnly = 3,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PixelBitmask {
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    reserved_mask: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ModeInformation {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
    pub pixel_information: PixelBitmask,
    pub pixels_per_scanline: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Mode {
    max_mode: u32,
    mode: u32,
    info: *mut ModeInformation,
    size_of_info: usize,
    framebuffer_base: *mut u8,
    framebuffer_size: usize,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum BltOperation {
    VideoFill = 0,
    VideoToBuffer = 1,
    BufferToVideo = 2,
    VideoToVideo = 3,
}

#[repr(C)]
pub struct GraphicsOutputProtocol {
    query_mode: unsafe extern "win64" fn(*const GraphicsOutputProtocol, mode_number: u32, size_of_info: *mut usize, info: *mut *mut ModeInformation) -> Status,
    set_mode: unsafe extern "win64" fn(*const GraphicsOutputProtocol, mode_number: u32) -> Status,
    blt: unsafe extern "win64" fn(*const GraphicsOutputProtocol, buffer: *mut Pixel, operation: BltOperation, source_x: usize, source_y: usize, dest_x: usize, dest_y: usize, width: usize, height: usize, delta: usize) -> Status,
    mode: *mut Mode,
}

impl ::Protocol for GraphicsOutputProtocol {
    fn guid() -> &'static Guid {
        return &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
    }
}

impl GraphicsOutputProtocol {
    pub fn query_mode(&self, mode_number: u32) -> Result<ModeInformation, Status> {
        let mut sz : usize = 0;
        let mut mip : *mut ModeInformation = ptr::null_mut();

        let res = unsafe { (self.query_mode)(self, mode_number, &mut sz, &mut mip) };
        if res != Status::Success {
            return Err(res);
        }

        // copy/free it
        let mi = unsafe { *mip };
        ::systemtable::get_system_table().boot_services().free_pool::<ModeInformation>(mip);

        Ok(mi)
    }

    pub fn set_mode(&self, mode_number: u32) -> Status {
        let status = unsafe { (self.set_mode)(self, mode_number) };
        return status;
    }

    pub fn draw(&self, pixels: &[Pixel], dest_x: usize, dest_y: usize, width: usize, height: usize) -> Status {
        // assuming BlueGreenRed here for delta
        unsafe {
            let px = pixels.as_ptr() as *mut Pixel;
            (self.blt)(self, px, BltOperation::BufferToVideo, 0, 0, dest_x, dest_y, width, height, 0)
        }
    }
}

