use ini::Ini;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::collections::HashMap;

pub fn init_ext_map() {
	let conf = Ini::load_from_file("sentire.ini").unwrap();
	let fmts = conf.section(Some("format")).unwrap().get("image").unwrap().split(';');
	let mut ext_map = HashMap::new();
	for fmt in fmts {
		ext_map.insert(fmt.to_string(), 1);
	}
	ext_map;
}

pub fn u2w(u8str: &str) -> *const u16 {
	OsStr::new(u8str).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr()
}
