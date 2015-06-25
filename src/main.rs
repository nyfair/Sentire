mod fi;
mod utils;

extern crate libc;
extern crate ini;
extern crate sdl2;
use sdl2::event::{Event, EventType};
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

fn draw_iamge(fname: String, renderer: &mut Renderer) {
	renderer.clear();
	let img = fi::Bitmap::open(&fname);
	let w = img.width();
	let h = img.height();
	let bpp = img.bpp();
	let pitch = img.pitch();
	let img_bits = img.raw();

	let image = Surface::from_data(img_bits, w, h, pitch, PixelFormatEnum::BGR24).unwrap();
    let texture = renderer.create_texture_from_surface(&image).unwrap();
	renderer.copy_ex(&texture, None, Some(Rect::new_unwrap(0, 0, w, h)), 0.0, None, (false, true));
	renderer.present();
}

fn main() {
	let ext_map = utils::init_ext_map();
	let mut sdl_context = sdl2::init().video().unwrap();
	sdl_context.event_pump().enable_event(EventType::DropFile);
	let window = sdl_context.window("sentire", 800, 600)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
	let mut renderer = window.renderer().build().unwrap();
	renderer.present();

	for event in sdl_context.event_pump().wait_iter() {
		match event {
			Event::DropFile { filename: fname, .. } => draw_iamge(fname, &mut renderer),
			Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break,
			_ => {}
		}
	}
}
