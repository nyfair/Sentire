use utils;

use libc::{c_int, c_void};

use std::slice;

#[link(name = "freeimage")]
extern "C" {
	fn FreeImage_LoadU(fif: u8, fname: *const u16, flags: c_int) -> *mut c_void;
	fn FreeImage_SaveU(fif: u8, img: *mut Bitmap, fname: *const u16, flags: c_int) -> i8;
	fn FreeImage_GetFileTypeU(fname: *const u16) -> u8;
	fn FreeImage_GetBPP(img: *mut c_void) -> u32;
	fn FreeImage_GetWidth(img: *mut c_void) -> u32;
	fn FreeImage_GetHeight(img: *mut c_void) -> u32;
	fn FreeImage_GetPitch(img: *mut c_void) -> u32;
	fn FreeImage_GetBits(img: *mut c_void) -> *mut u8;
	fn FreeImage_Unload(img: *mut c_void);
}

#[repr(C)]
pub struct Bitmap {
	img: *mut c_void,
}

impl Drop for Bitmap {
	fn drop(&mut self) {
		unsafe { FreeImage_Unload(self.img) }
	}
}

impl Bitmap {
	pub fn open(fname: &str) -> Bitmap {
		let u16name = utils::u2w(&fname);
		unsafe {
			let dib = FreeImage_LoadU(FreeImage_GetFileTypeU(u16name), u16name, 0);
			return Bitmap{img: dib};
		}
	}

	pub fn bpp(&self) -> u32 {
		unsafe { FreeImage_GetBPP(self.img) }
	}

	pub fn width(&self) -> u32 {
		unsafe { FreeImage_GetWidth(self.img) }
	}

	pub fn height(&self) -> u32 {
		unsafe { FreeImage_GetHeight(self.img) }
	}

	pub fn pitch(&self) -> u32 {
		unsafe { FreeImage_GetPitch(self.img) }
	}

	pub fn raw(&self) -> &mut[u8] {
		let size = self.width() * self.pitch();
		unsafe { slice::from_raw_parts_mut(FreeImage_GetBits(self.img), size as usize) }
	}
}
