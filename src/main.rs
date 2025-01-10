use rand::Rng;
use std::{thread, time};
use colored::{Colorize, CustomColor};
use CLAAR::primatives::{Point, TriColorer, Triangle};
use CLAAR::framebuffer::FrameBuffer;

//The included binary is an example of how to use the library

const FRAME_WAIT_MS : u64 = 15;

fn uv_as_rg(x : f64, y : f64, uv_x : f64, uv_y : f64) -> (f64, f64, f64)
{
	return (uv_x, uv_y, 0_f64)
}

fn uv_as_rb(x : f64, y : f64, uv_x : f64, uv_y : f64) -> (f64, f64, f64)
{
	return (uv_x,  0_f64, uv_y)
}

fn main() -> Result<(), std::io::Error>
{
	let mut main_buffer : FrameBuffer = FrameBuffer::new(165, 90, String::from("Main Display") );
	let mut time : f64 = 0_f64;

	let mut rand_triangle : Triangle = Triangle::new
	(
		Point::new
		(
			rand::thread_rng().gen_range(0_f64..main_buffer.get_width() as f64),
			rand::thread_rng().gen_range(0_f64..main_buffer.get_height() as f64),
		),
		Point::new
		(
			rand::thread_rng().gen_range(0_f64..main_buffer.get_width() as f64),
			rand::thread_rng().gen_range(0_f64..main_buffer.get_height() as f64),
		),
		Point::new
		(
			rand::thread_rng().gen_range(0_f64..main_buffer.get_width() as f64),
			rand::thread_rng().gen_range(0_f64..main_buffer.get_height() as f64),
		),

		Some(uv_as_rb)
	);

	let rand_tri_mid : Point = Point::new
	(
		(rand_triangle.get_point(0).x + rand_triangle.get_point(1).x + rand_triangle.get_point(2).x) / 3_f64,
		(rand_triangle.get_point(0).y + rand_triangle.get_point(1).y + rand_triangle.get_point(2).y) / 3_f64,
	);

	let upper_left : Triangle = Triangle::new
	(
		Point::new(0_f64, 0_f64), Point::new(0_f64, main_buffer.get_height() as f64),
		Point::new(main_buffer.get_width() as f64, main_buffer.get_height() as f64), Some(uv_as_rg)
	);

	let lower_right : Triangle = Triangle::new
	(
		Point::new(0_f64, 0_f64),
		Point::new(main_buffer.get_width() as f64, 0_f64),
		Point::new(main_buffer.get_width() as f64, main_buffer.get_height() as f64),
		 Some(uv_as_rg)
	);

	loop
	{
		main_buffer.draw_triangle(&lower_right);
		main_buffer.draw_triangle(&upper_left);
		main_buffer.draw_triangle(&rand_triangle);

		main_buffer.draw_buffer();

		// println!("Triangle byte size: {}", size_of::<Triangle>());
		// println!("Triangle width: {}", rand_triangle.width);
		// println!("Triangle height: {}", rand_triangle.height);


		rand_triangle.set_point(0, rand_triangle.get_point(0).rotate_point(&rand_tri_mid, 0.05_f64));
		rand_triangle.set_point(1, rand_triangle.get_point(1).rotate_point(&rand_tri_mid, 0.05_f64));
		rand_triangle.set_point(2, rand_triangle.get_point(2).rotate_point(&rand_tri_mid, 0.05_f64));

		thread::sleep(time::Duration::from_millis(FRAME_WAIT_MS)); //Cool down to not overwhelm terminal
		time += 0.0001_f64;
		main_buffer.clear_buffer();
	}
	
	return Ok(());
}
