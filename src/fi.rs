use libc::{c_int, c_void, c_char};
use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;

#[link(name = "freeimage")]
extern "C" {
	pub fn FreeImage_LoadU(fif: u8, filename: *const u16, flags: c_int) -> *mut c_void;
	pub fn FreeImage_SaveU(fif: u8, img: *mut c_void, filename: *const u16, flags: c_int) -> i8;
	pub fn FreeImage_GetFileTypeU(filename: *const u16) -> u8;
	pub fn FreeImage_GetBPP(img: *mut c_void) -> u32;
	pub fn FreeImage_GetWidth(img: *mut c_void) -> u32;
	pub fn FreeImage_GetHeight(img: *mut c_void) -> u32;
	pub fn FreeImage_GetBits(img: *mut c_void) -> *mut u8;
	pub fn FreeImage_Unload(img: *mut c_void) -> c_void;
}

pub fn u2w(u8str: &str) -> *const u16 {
	return OsStr::new(u8str).encode_wide().collect::<Vec<_>>().as_ptr();
}
