// A struct representing the framebuffer's contents
use crate::primatives::Point;
use crate::primatives::Triangle;
use colored::{Color, Colorize, CustomColor};
use terminal_size::{Width, Height, terminal_size};

const PIXEL_CHAR : &str = "  ";

#[derive(Clone)]
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
    pub fn new(r : f64, g : f64, b : f64, a : f64) -> Self
    {
        return Self
        {
            r : r,
            g : g,
            b : b,
            a : a,
        };
    }
}

pub struct FrameBuffer
{
    //A buffer for holding which characters are in what state
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
            content : vec![vec![ Tile::new(0_f64, 0_f64, 0_f64, 1_f64) ; width] ; height], //vectors are needed bc rust doesnt have variable array lengths >:(
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
        for (row_num, row) in (1_usize..=self.height).rev().zip(self.content.iter().rev())
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

        let prev_tile : Tile = Tile::new
        (
            self.content[point.y as usize][point.x as usize].r,
            self.content[point.y as usize][point.x as usize].g,
            self.content[point.y as usize][point.x as usize].b,
            self.content[point.y as usize][point.x as usize].a
        );

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
        let x_lowest_point : Point = triangle.get_point(triangle.lowest_x.0.unwrap() as usize);
        let x_highest_point : Point = triangle.get_point(triangle.highest_x.0.unwrap() as usize);
        let neither_x_extreme_point : Point = triangle.get_point((3 - triangle.lowest_x.0.unwrap() -  triangle.highest_x.0.unwrap()) as usize);
        let neither_x_exists : bool = (triangle.lowest_x.1 != None) || (triangle.highest_x.1 != None); //needed for edge cases with tris with 2 points at the same x coord
        let num_tri_floor_pieces : u8 = if neither_x_extreme_point.is_above_line(&x_lowest_point, &x_highest_point) || neither_x_exists { 1 } else { 2 };
        //^^^ If the owner of neither x extreme is above the other two points, it's a one piece floor

        //Go through all points within triangle extreme and determine if they lie within the tri
        let lowest_y : f64 = triangle.get_point(triangle.lowest_y.0.unwrap() as usize).y;
        let highest_y : f64 = triangle.get_point(triangle.highest_y.0.unwrap() as usize).y;
        let lowest_x : f64 = triangle.get_point(triangle.lowest_x.0.unwrap() as usize).x;
        let highest_x : f64 = triangle.get_point(triangle.highest_x.0.unwrap() as usize).x;

        for y in (f64::max(lowest_y, 0_f64)) as usize..=(f64::min(highest_y, (self.height - 1) as f64)) as usize
        {
            for x in (f64::max(lowest_x, 0_f64)) as usize..=(f64::min(highest_x, (self.width - 1) as f64)) as usize
            {
                const NUM_DIVS : u16 = 1;
                let mut num_valid_divs : u32 = 0;

                //Tile color to be applied to be colored in tile
                let mut tile : Tile =
                match triangle.colorer
                {
                    Some(colorer) => colorer(x as f64, y as f64, (x as f64 - lowest_x) / triangle.width, (y as f64 - lowest_y) / triangle.height),
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

                        let mut above_below_checks : u8 = 0; //number of how many lines created by tri the point is above
                        above_below_checks += tested.is_above_line(&triangle.get_point(0), &triangle.get_point(1)) as u8;
                        above_below_checks += tested.is_above_line(&triangle.get_point(0), &triangle.get_point(2)) as u8;
                        above_below_checks += tested.is_above_line(&triangle.get_point(1), &triangle.get_point(2)) as u8;

                        if above_below_checks == num_tri_floor_pieces
                        {
                            num_valid_divs += 1;
                        }
                    }
                }

                if num_valid_divs == 0
                {
                    continue;
                }

                let pixel_fitness : f64 = num_valid_divs as f64 / (NUM_DIVS * NUM_DIVS) as f64;

                tile.a *= pixel_fitness;

                self.draw_point( &Point{x : x as f64, y : y as f64}, tile)
            }
        }
    }

    pub fn clear_buffer(self : &mut Self)
    {
        self.content = vec![ vec![ Tile::new(0_f64, 0_f64, 0_f64, 1_f64) ; self.width ] ; self.height ];
    }
}
