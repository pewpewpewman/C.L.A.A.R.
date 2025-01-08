use rand::Rng;
use std::{thread, time};
use CLAAR::primatives::{Point, Triangle};
use CLAAR::framebuffer::FrameBuffer;

//The included binary is an example of how to use the library

const FRAME_WAIT_MS : u64 = 20;

fn main() -> Result<(), std::io::Error>
{
	let mut main_buffer : FrameBuffer = FrameBuffer::new(170, 80, String::from("Main Display") );
	let mut time : f64 = 0_f64;

	let mut og_triangle : Triangle = Triangle::new
	(
		Point::new
		(
			rand::thread_rng().gen_range(0_f64..main_buffer.get_width() as f64),
			rand::thread_rng().gen_range(0_f64..main_buffer.get_height() as f64)
		),
		Point::new
		(
			rand::thread_rng().gen_range(0_f64..main_buffer.get_width() as f64),
			rand::thread_rng().gen_range(0_f64..main_buffer.get_height() as f64)
		),
		Point::new
		(
			rand::thread_rng().gen_range(0_f64..main_buffer.get_width() as f64),
			rand::thread_rng().gen_range(0_f64..main_buffer.get_height() as f64)
		),
	);

	let tri_mid : Point = Point::new
	(
		(og_triangle.get_point(0).x + og_triangle.get_point(1).x + og_triangle.get_point(2).x) / 3_f64,
		(og_triangle.get_point(0).y + og_triangle.get_point(1).y + og_triangle.get_point(2).y) / 3_f64,
	);

	loop
	{
		main_buffer.draw_triangle(&og_triangle);
		main_buffer.draw_buffer();

		og_triangle.set_point(0, og_triangle.get_point(0).rotate_point(&tri_mid, 0.05_f64));
		og_triangle.set_point(1, og_triangle.get_point(1).rotate_point(&tri_mid, 0.05_f64));
		og_triangle.set_point(2, og_triangle.get_point(2).rotate_point(&tri_mid, 0.05_f64));

		thread::sleep(time::Duration::from_millis(FRAME_WAIT_MS)); //Cool down to not overwhelm terminal
		time += 0.0001_f64;
		main_buffer.clear_buffer();
	}
	
	return Ok(());
}
