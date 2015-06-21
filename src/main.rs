mod fi;
mod utils;

extern crate libc;
extern crate ini;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keycode::KeyCode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use std::env;

fn main() {
	let ext_map = utils::init_ext_map();
	let fname = env::args().nth(1).unwrap();

	let img = fi::open(utils::u2w(&fname));
	let w = fi::width(img);
	let h = fi::height(img);
	let bpp = fi::bpp(img);
	let pitch = w*3;
	let img_bits = fi::raw(img);
	let img_ptr = unsafe { &mut Vec::from_raw_parts(img_bits, (w*h*3) as usize, (w*h*3) as usize) };

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
	fi::free(img);
}
