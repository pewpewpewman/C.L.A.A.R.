// A struct representing the framebuffer's contents
use crate::primatives::Point;
use crate::primatives::Triangle;

const ENABLE_GRID_MARKERS : bool = false;
const GRID_MARKER_INTERVAL : u8 = 5; //make sure this is some value greater than 1
const BLANK : char = '□';
const FILL : char =  '■';

pub struct FrameBuffer
{
    //A buffer for holding which characters are in what state
    content : Vec<Vec<char>>,

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
            content : vec![ vec![BLANK ; width] ; height], //vectors are needed bc rust doesnt have variable array lengths >:(
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

        //This might seem random, but it's really the best number for drawing the top row of dashes
        let bottom_dash_num : usize = self.width;
        let top_dash_num : usize = bottom_dash_num - (self.name.len() / 3); //this looks *FINE* ig
        assert!(top_dash_num > 0, "Draw Buffer name is too long!");

        //Draw Top Line
        print!(" +");
        print!("{}", " -".repeat(top_dash_num / 2));
        print!("{}", self.name);
        print!("{}", "- ".repeat(top_dash_num / 2));
        println!("+");

        //Draw Buffer Contents with Border
        for (row_num, row) in (1_usize..=self.height).rev().zip(self.content.iter().rev())
        {
            //Handle row starting number / line
            let mut row_starter : [char; 2] = [' ', ' '];
            if ENABLE_GRID_MARKERS && (row_num as u8 % GRID_MARKER_INTERVAL == 0_u8 || row_num == 1)
            {
                if row_num < 10
                {
                    row_starter[0] = ' ';
                    row_starter[1] = char::from_digit(row_num as u32, 10).unwrap();
                }
                else
                {
                    row_starter[0] = char::from_digit((row_num / 10) as u32, 10).unwrap();
                    row_starter[1] = char::from_digit((row_num % 10) as u32, 10).unwrap();
                }
            }
            else
            {
                row_starter = [' ', '|'];
            }
            print!("{}", String::from_iter(row_starter));

            //Draw row contents
            for tile in row
            {
                print!(" {}", tile); //Each tile starst w/ space to make an even square
            }

            //Handle row ending line
            println!(" |"); // Leading space for squareness
        }

        //Draw Bottom Line
        print!(" +");
        for column in 1_u8..=bottom_dash_num as u8
        {
            //Handle row starting number / line
            //Horrible bool pathing - whatever
            let mut column_base : [char; 2] = [' ', ' '];
            if ENABLE_GRID_MARKERS && column < 10 && (column % GRID_MARKER_INTERVAL == 0 || column == 1)
            {
                column_base[0] = ' ';
                column_base[1] = char::from_digit(column as u32, 10).unwrap();
            }
            else if ENABLE_GRID_MARKERS && column % GRID_MARKER_INTERVAL == 0
            {
                column_base[0] = ' ';
                column_base[1] = char::from_digit((column / 10) as u32, 10).unwrap();
            }
            else if ENABLE_GRID_MARKERS && column % GRID_MARKER_INTERVAL == 1 && column > 10
            {
                column_base[0] = char::from_digit(((column - 1) % 10) as u32, 10).unwrap();
                column_base[1] = '-';
            }
            else
            {
                column_base = [' ', '-'];
            }
            print!("{}", String::from_iter(column_base));
        }

        //All of this is to account for the ending " +" cutting off the end of the last number
        //It only gets vut off if its double digits and divisble by five
        let mut bottom_right : [char; 2] = [' ', '+'];
        if ENABLE_GRID_MARKERS && bottom_dash_num > 10 && bottom_dash_num as u8 % GRID_MARKER_INTERVAL == 0_u8
        {
            bottom_right[0] = char::from_digit((bottom_dash_num % 10) as u32, 10).unwrap();
        }
        println!("{}", String::from_iter(bottom_right));

        //thread::sleep(time::Duration::from_millis(FRAME_WAIT_MS)); //Cool down to not overwhelm terminal
    }

    pub fn draw_point(self : &mut Self, point : &Point) -> ()
    {
        //assert!((point.x as usize) < width, "X POINT VALUE TOO LARGE");
        //assert!((point.y as usize) < height, "Y POINT VALUE TOO LARGE");
        //Prevent the drawing of points that are out of range
        if point.x >= self.width as f64 || point.y >= self.height as f64 || point.x < 0_f64 || point.y < 0_f64
        {
            return;
        }

        self.content[point.y as usize][point.x as usize] = FILL;
    }

    pub fn draw_triangle(self : &mut Self, triangle : &Triangle) -> ()
    {
        let x_lowest_point : Point = triangle.get_point(triangle.lowest_x.0.unwrap() as usize);
        let x_highest_point : Point = triangle.get_point(triangle.highest_x.0.unwrap() as usize);
        let neither_x_extreme_point : Point = triangle.get_point((3 - triangle.lowest_x.0.unwrap() -  triangle.highest_x.0.unwrap()) as usize);
        let neither_x_exists : bool = triangle.lowest_x.1 == None && triangle.highest_x.1 == None; //needed for edge cases with right triangles
        let mut num_tri_floor_pieces : u8 = //honestly could be a bool but who cares
        {
            //If the owner of niether x extreme is above the other two points, its a one piece floor
            if neither_x_extreme_point.is_above_line(&x_lowest_point, &x_highest_point) || neither_x_exists
            {
                1
            }
            else
            {
                2
            }
        };


        //Go through all points within triangle extreme and determine if they lie within the tri
        let lowest_y : f64 = triangle.get_point(triangle.lowest_y.0.unwrap() as usize).y;
        let highest_y : f64 = triangle.get_point(triangle.highest_y.0.unwrap() as usize).y;
        let lowest_x : f64 = triangle.get_point(triangle.lowest_x.0.unwrap() as usize).x;
        let highest_x : f64 = triangle.get_point(triangle.highest_x.0.unwrap() as usize).x;

        for y in (f64::max(lowest_y, 0_f64)) as usize..=(f64::min(highest_y, (self.height - 1) as f64)) as usize
        {
            for x in (f64::max(lowest_x, 0_f64)) as usize..=(f64::min(highest_x, (self.width - 1) as f64)) as usize
            {
                const NUM_TILE_DIVS : u8 = 32;

                //Split each tile into smaller tiles and if any of them work, the whole tile is drawn
                for x_div in 0..=(NUM_TILE_DIVS / 2)
                {
                    for y_div in 0..=(NUM_TILE_DIVS / 2)
                    {
                        let tested : Point = Point
                        {
                            x : x as f64 + (x_div as f64 / NUM_TILE_DIVS as f64),
                            y : y as f64 + (y_div as f64 / NUM_TILE_DIVS as f64),
                        };

                        let mut above_below_checks : u8 = 0; //number of how many lines created by tri the point is above

                        above_below_checks += tested.is_above_line(&triangle.get_point(0), &triangle.get_point(1)) as u8;
                        above_below_checks += tested.is_above_line(&triangle.get_point(0), &triangle.get_point(2)) as u8;
                        above_below_checks += tested.is_above_line(&triangle.get_point(1), &triangle.get_point(2)) as u8;

                        if above_below_checks == num_tri_floor_pieces
                        {
                            self.draw_point(&tested);
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn clear_buffer(self : &mut Self)
    {
        for mut row in &mut self.content
        {
            row.fill(BLANK);
        }
    }
}
