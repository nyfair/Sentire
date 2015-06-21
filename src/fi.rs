use libc::{c_int, c_void};

#[link(name = "freeimage")]
extern "C" {
	fn FreeImage_LoadU(fif: u8, fname: *const u16, flags: c_int) -> *mut c_void;
	fn FreeImage_SaveU(fif: u8, img: *mut c_void, fname: *const u16, flags: c_int) -> i8;
	fn FreeImage_GetFileTypeU(fname: *const u16) -> u8;
	fn FreeImage_GetBPP(img: *mut c_void) -> u32;
	fn FreeImage_GetWidth(img: *mut c_void) -> u32;
	fn FreeImage_GetHeight(img: *mut c_void) -> u32;
	fn FreeImage_GetBits(img: *mut c_void) -> *mut u8;
	fn FreeImage_Unload(img: *mut c_void) -> c_void;
}

pub fn open(fname: *const u16) -> *mut c_void {
	unsafe { FreeImage_LoadU(FreeImage_GetFileTypeU(fname), fname, 0) }
}

pub fn free(img: *mut c_void) -> c_void {
	unsafe { FreeImage_Unload(img) }
}

pub fn raw(img: *mut c_void) -> *mut u8 {
	unsafe { FreeImage_GetBits(img) }
}

pub fn bpp(img: *mut c_void) -> u32 {
	unsafe { FreeImage_GetBPP(img) }
}

pub fn width(img: *mut c_void) -> u32 {
	unsafe { FreeImage_GetWidth(img) }
}

pub fn height(img: *mut c_void) -> u32 {
	unsafe { FreeImage_GetHeight(img) }
}
