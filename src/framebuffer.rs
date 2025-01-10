// A struct representing the framebuffer's contents
use crate::primatives::Point;
use crate::primatives::Triangle;
use colored::{Color, Colorize, CustomColor};
use colored::Color::{Black, TrueColor};

const ENABLE_GRID_MARKERS : bool = false;
const GRID_MARKER_INTERVAL : u8 = 5; //make sure this is some value greater than 1
const PIXEL_CHAR : &str = "  "; //"■";

pub struct FrameBuffer
{
    //A buffer for holding which characters are in what state
    content : Vec<Vec<Color>>,

    //Width and Height of buffer
    width : usize,
    height : usize,

    //Name to be drawn above the "screen"
    name : String,
}

impl FrameBuffer
{
    pub fn new(width : usize, height : usize, name : String) -> Self
    {
        return Self
        {
            width : width,
            height : height,
            content : vec![vec![ Color::TrueColor {r : 0, g : 0, b: 0} ; width] ; height], //vectors are needed bc rust doesnt have variable array lengths >:(
            name : name,
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
        for (row_num, row) in (1_usize..=self.height).rev().zip(self.content.iter().rev())
        {
            //Draw row contents
            for tile in row
            {
                if let Color::TrueColor {r, g, b} = tile
                {
                    print!("{}", PIXEL_CHAR.on_truecolor(*r, *g, *b)); //Each tile starts w/ space to make an even square
                }
            }
            println!("");
        }
    }

    pub fn draw_point(self : &mut Self, point : &Point, col : Color) -> ()
    {
        //assert!((point.x as usize) < width, "X POINT VALUE TOO LARGE");
        //assert!((point.y as usize) < height, "Y POINT VALUE TOO LARGE");
        //Prevent the drawing of points that are out of range
        if point.x >= self.width as f64 || point.y >= self.height as f64 || point.x < 0_f64 || point.y < 0_f64
        {
            return;
        }
        if let Color::TrueColor {r, g, b} = col
        {
            self.content[point.y as usize][point.x as usize] = Color::TrueColor { r: r, g: g, b: b };
        }
    }

    pub fn draw_triangle(self : &mut Self, triangle : &Triangle) -> ()
    {
        let x_lowest_point : Point = triangle.get_point(triangle.lowest_x.0.unwrap() as usize);
        let x_highest_point : Point = triangle.get_point(triangle.highest_x.0.unwrap() as usize);
        let neither_x_extreme_point : Point = triangle.get_point((3 - triangle.lowest_x.0.unwrap() -  triangle.highest_x.0.unwrap()) as usize);
        let neither_x_exists : bool = (triangle.lowest_x.1 != None) || (triangle.highest_x.1 != None); //needed for edge cases with tris with 2 points at the same x coord
        let num_tri_floor_pieces : u8 =
        if neither_x_extreme_point.is_above_line(&x_lowest_point, &x_highest_point) || neither_x_exists
        {
            1
        }
        else
        {
            2
        };
        //^^^ If the owner of neither x extreme is above the other two points, it's a one piece floor
        println!("niether x existss??? {neither_x_exists}");

        //Go through all points within triangle extreme and determine if they lie within the tri
        let lowest_y : f64 = triangle.get_point(triangle.lowest_y.0.unwrap() as usize).y;
        let highest_y : f64 = triangle.get_point(triangle.highest_y.0.unwrap() as usize).y;
        let lowest_x : f64 = triangle.get_point(triangle.lowest_x.0.unwrap() as usize).x;
        let highest_x : f64 = triangle.get_point(triangle.highest_x.0.unwrap() as usize).x;

        for y in (f64::max(lowest_y, 0_f64)) as usize..=(f64::min(highest_y, (self.height - 1) as f64)) as usize
        {
            for x in (f64::max(lowest_x, 0_f64)) as usize..=(f64::min(highest_x, (self.width - 1) as f64)) as usize
            {
                const NUM_DIVS : u8 = 1;
                let mut num_valid_divs : u32 = 0;

                //Split each tile into smaller tiles and if any of them work, the whole tile is drawn
                for x_div in 0..=NUM_DIVS
                {
                    for y_div in 0..=NUM_DIVS
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

                let pixel_fitness : f64 = num_valid_divs as f64 / ((NUM_DIVS * NUM_DIVS) as f64);
                let col : Color = match triangle.colorer
                {
                    Some(colorer) =>
                    {
                        let (r, g, b) : (f64, f64, f64) = colorer
                        (
                            x as f64,
                            y as f64,
                            (x as f64 - lowest_x) / triangle.width,
                            (y as f64 - lowest_y) / triangle.height,
                        );

                        Color::TrueColor
                        {
                            r : (r * 255_f64) as u8,
                            g : (g * 255_f64) as u8,
                            b : (b * 255_f64) as u8,
                        }
                    }

                    None =>
                    {
                        Color::TrueColor
                        {
                            r : (255_f64 * pixel_fitness) as u8,
                            g : (255_f64 * pixel_fitness) as u8,
                            b : (255_f64 * pixel_fitness) as u8,
                        }
                    }
                };

                self.draw_point( &Point{x : x as f64, y : y as f64}, col)
            }
        }
    }

    pub fn clear_buffer(self : &mut Self)
    {
        self.content.fill(vec![  Color::TrueColor {r : 0, g : 0, b: 0} ; self.width ]);
    }
}
