mod fi;

extern crate libc;

extern crate ini;
use ini::Ini;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keycode::KeyCode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use std::collections::HashMap;
use std::env;
use std::mem;

fn init_ext_map(ext_map: &mut HashMap<String, i8>) {
	let conf = Ini::load_from_file("sentire.ini").unwrap();
	let fmts = conf.section(Some("Image")).unwrap().get("extensions").unwrap().split(';');
	for fmt in fmts {
		ext_map.insert(fmt.to_string(), 1);
	}
}

fn main() {
unsafe {
	let fname = env::args().nth(1).unwrap();
	let mut ext_map = HashMap::new();
	init_ext_map(&mut ext_map);

	let fif = fi::FreeImage_GetFileTypeU(fi::u2w(&fname));
	let img = fi::FreeImage_LoadU(fif, fi::u2w(&fname), 0);
	let w = fi::FreeImage_GetWidth(img);
	let h = fi::FreeImage_GetHeight(img);
	let bpp = fi::FreeImage_GetBPP(img);
	let pitch = w*3;
	let img_bits = fi::FreeImage_GetBits(img);
	let img_ptr = &mut Vec::from_raw_parts(img_bits, (w*h*3) as usize, (w*h*3) as usize);

	let mut sdl_context = sdl2::init().video().unwrap();
	let window = sdl_context.window("sentire", w, h)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
	let mut renderer = window.renderer().build().unwrap();
	let image = Surface::from_data(img_ptr, w, h, pitch, PixelFormatEnum::BGR24).unwrap();
    let texture = renderer.create_texture_from_surface(&image).unwrap();

	let mut drawer = renderer.drawer();
	drawer.copy_ex(&texture, None, Some(Rect::new(0, 0, w as i32, h as i32)), 0.0, None, (false, true));
	drawer.present();

	let mut running = true;
	while running {
		for event in sdl_context.event_pump().poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
					running = false
				},
				_ => {}
			}
		}
	}
	fi::FreeImage_Unload(img);
}}
