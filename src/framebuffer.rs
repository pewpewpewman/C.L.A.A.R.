// A struct representing the framebuffer's contents
use crate::primatives::Point;
use crate::primatives::Triangle;
use colored::{Color, Colorize, CustomColor};
use terminal_size::{Width, Height, terminal_size};

const PIXEL_CHAR : &str = "  ";

#[derive(Clone, PartialEq)]
pub struct Tile
{
    r : f64,
    g : f64,
    b : f64,
    a : f64
}

//Type representing each tile of the buffer
//An alpha value of one represents fully opaque
impl Tile
{
    pub fn new(r : f64, g : f64, b : f64, a : f64) -> Tile
    {
        return Tile
        {
            r : r,
            g : g,
            b : b,
            a : a,
        };
    }
}

impl Default for Tile
{
	fn default() -> Tile
	{
		return Tile::new(0_f64, 0_f64, 0_f64, 0_f64);
	}
}

pub struct FrameBuffer
{
    //A buffer for holding which tiles are in what state
    content : Vec<Vec<Tile>>,

    //Width and Height of buffer
    width : usize,
    height : usize,

    //Text data used for drawing
    vert_border_tile : String,
    horiz_border : String,
}

impl FrameBuffer
{
    pub fn new() -> Self
    {
        let (width, height) : (usize, usize);
        match terminal_size()
        {
            Some((w, h)) => { width = (w.0 / 2 - 2) as usize; height = (h.0 - 4) as usize; },
            None => { panic!("Could not get terminal size"); }
        };

        return Self
        {
            width : width,
            height : height,
            content : vec![ vec![ Tile::new(0_f64, 0_f64, 0_f64, 1_f64) ; width] ; height], //vectors are needed bc rust doesnt have variable array lengths >:(
            vert_border_tile : format!("{}", "|".on_black()),
            horiz_border : format!("{}{}{} ", "+", "-".repeat(width * 2).on_black(), "+")
        };
    }
    
    pub fn get_width(&self) -> usize
    {
        return self.width;
    }

    pub fn get_height(&self) -> usize
    {
        return self.height;
    }

    pub fn draw_buffer(self : &Self) -> ()
    {
        std::process::Command::new("clear").status().unwrap(); //Wipe previous draw

        //Draw Buffer Contents with Border
        println!("{}", self.horiz_border);
        for row in self.content.iter().rev()
        {
            //Draw row contents
            print!("{}", self.vert_border_tile);
            for tile in row
            {
                print!("{}", PIXEL_CHAR.on_truecolor( (255_f64 * tile.r) as u8, (255_f64 * tile.g) as u8, (255_f64 * tile.b) as u8 ));
            }
            println!("{}", self.vert_border_tile);
        }
        println!("{}", self.horiz_border);
    }

    pub fn draw_point(self : &mut Self, point : &Point, tile : Tile) -> ()
    {
        //assert!((point.x as usize) < width, "X POINT VALUE TOO LARGE");
        //assert!((point.y as usize) < height, "Y POINT VALUE TOO LARGE");
        //Prevent the drawing of points that are out of range
        
        if point.x >= self.width as f64 || point.y >= self.height as f64 || point.x < 0_f64 || point.y < 0_f64
        {
            return;
        }

        let mut prev_tile : Tile = self.content[point.y as usize][point.x as usize].clone();

		//Removes blending if previous pixel was totally black, gives better looking base canvas
		if (prev_tile == Tile::new(0_f64, 0_f64, 0_f64, prev_tile.a))
		{
			prev_tile = tile.clone();
		}
		
        let new_tile : Tile = Tile::new
        (
            tile.r * tile.a + prev_tile.r * (1_f64 - tile.a),
            tile.g * tile.a + prev_tile.g * (1_f64 - tile.a),
            tile.b * tile.a + prev_tile.b * (1_f64 - tile.a),
            tile.a
        );

        self.content[point.y as usize][point.x as usize] = new_tile;
    }

    pub fn draw_triangle(self : &mut Self, triangle : &Triangle) -> ()
    {
        //Go through all points within triangle extreme and determine if they lie within the tri
        let lowest_y : f64 = triangle.get_point(triangle.lowest_y.0.unwrap() as usize).y;
        let highest_y : f64 = triangle.get_point(triangle.highest_y.0.unwrap() as usize).y;
        let lowest_x : f64 = triangle.get_point(triangle.lowest_x.0.unwrap() as usize).x;
        let highest_x : f64 = triangle.get_point(triangle.highest_x.0.unwrap() as usize).x;

        for y in (f64::max(lowest_y, 0_f64)) as usize..=(f64::min(highest_y, (self.height - 1) as f64)) as usize
        {
            for x in (f64::max(lowest_x, 0_f64)) as usize..=(f64::min(highest_x, (self.width - 1) as f64)) as usize
            {
                const NUM_DIVS : u16 = 4;
                let mut num_valid_divs : u32 = 0;

                //Tile color to be applied to be colored in tile
                let mut tile : Tile =
                match &triangle.colorer
                {
                    Some(colorer) =>
                    {
                    	let (w1, w2, w3) : (f64, f64, f64) = triangle.calc_weights(&Point::new(x as f64, y as f64));
                    	let num_weighted_vals : u8 = triangle.coloring_data.len() as u8;
                    	let mut weighted_vals : Vec<f64> = vec![0_f64 ; num_weighted_vals as usize];
                    	for weight_idx in 0..num_weighted_vals
                    	{
                    		weighted_vals[weight_idx as usize] =
                    		{
                    			triangle.coloring_data[weight_idx as usize].0 * w1 +
                    			triangle.coloring_data[weight_idx as usize].1 * w2 +
                    			triangle.coloring_data[weight_idx as usize].2 * w3	
                    		}
                    	}
                    	colorer(Point::new((x as f64 - lowest_x) / triangle.width, (y as f64 - lowest_y) / triangle.height), &weighted_vals)
                    }
                    None => Tile::new(1_f64, 1_f64, 1_f64, 1_f64)
                };

                //Split each tile into smaller tiles and if any of them work, the whole tile is drawn
                for x_div in 1..=NUM_DIVS
                {
                    for y_div in 1..=NUM_DIVS
                    {
                        let tested : Point = Point
                        {
                            x : x as f64 + (x_div as f64 / NUM_DIVS as f64),
                            y : y as f64 + (y_div as f64 / NUM_DIVS as f64),
                        };

						let (w_1, w_2, w_3) : (f64, f64, f64) = triangle.calc_weights(&tested);
                        
                        if (w_1 >= 0_f64) && (w_2 >= 0_f64) && (w_3 >= 0_f64)
                        {
                            num_valid_divs += 1;
                        }
                    }
                }

                if num_valid_divs == 0
                {
                    continue;
                }
                				
                tile.a = (num_valid_divs as f64 / (NUM_DIVS * NUM_DIVS) as f64);
                                
                self.draw_point( &Point{x : x as f64, y : y as f64}, tile)
            }
        }
    }

    pub fn clear_buffer(self : &mut Self)
    {
        self.content = vec![ vec![ Tile::new(0_f64, 0_f64, 0_f64, 1_f64) ; self.width ] ; self.height ];
    }
}
