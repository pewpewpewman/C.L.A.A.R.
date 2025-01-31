use colored::{Color, CustomColor};
use float_cmp::approx_eq;
use crate::framebuffer;
use crate::framebuffer::FrameBuffer;
use crate::framebuffer::Tile;

#[derive(Debug, Copy, Clone)]
pub struct Point
{
    pub x : f64,
    pub y : f64,
}

impl Point
{

    pub fn new(x : f64, y : f64) -> Self
    {
        return Self
        {
            x : x,
            y : y,
        };
    }

    //uses point-slope form to find if point lies above a line made by two given points
    pub fn is_above_line(self : &Self, line_point_one : &Point, line_point_two : &Point) -> bool
    {
        if (line_point_two.x == line_point_one.x)
        {
            return false;
        }

        let slope : f64 = (line_point_two.y - line_point_one.y) / (line_point_two.x - line_point_one.x);
        return self.y > slope * (self.x - line_point_one.x) + line_point_one.y;
    }

    pub fn rotate_point(self : &Self, about : &Point, angle : f64) -> Point
    {
        let sin_of_angle = f64::sin(angle);
        let cos_of_angle = f64::cos(angle);
        return Point::new
        (
            ((self.x - about.x) * cos_of_angle - (self.y - about.y) * sin_of_angle) + about.x,
            ((self.x - about.x) * sin_of_angle + (self.y - about.y) * cos_of_angle) + about.y,
        );
    }

    pub fn add(self : &Self, other : Point) -> Point
    {
        return Point::new(self.x + other.x, self.y + other.y);
    }

    pub fn sub(self : &Self, other : Point) -> Point
    {
        return Point::new(self.x - other.x, self.y - other.y);
    }

}

//A triangle made up of three points. Contains extra information to assist in drawing
//It is very important that the triangle extremes are kept up to date with any changes made to the points
pub struct Triangle
{
    points : [Point; 3],
    //The *indexes* of which point has these titles
    //Having to constantly convert these to usizes for indexing is some shit
    //Represented as tuples becuase two points can share the same extreme
    //Option type used to represent the fact that each extreme can have just one point (and in most cases this is the case)
    //TODO: Refactor so only second item is option
    pub lowest_x : (Option<u8>, Option<u8>),
    pub highest_x : (Option<u8>, Option<u8>),
    pub lowest_y : (Option<u8>, Option<u8>),
    pub highest_y : (Option<u8>, Option<u8>),

    //width and height needed for calculating UVs in colorizer
    pub width : f64,
    pub height : f64,
    //Data interpolated along the triangle during coloring - all vecs must be equal in size
    coloring_data : Option<[ Vec<f64> ; 3]>,
    pub colorer : Option<TriColorer>
}

//Colorer functions color the triangle using the following four parameters:
//UV cord is always input, float array is interped by weights
pub type TriColorer = fn(Point, &[f64]) -> framebuffer::Tile;

impl Triangle
{
    pub fn new(point_one : (Point, Option<Vec<f64>>), point_two : (Point, Option<Vec<f64>>), point_three : (Point, Option<Vec<f64>>), colorer : Option<TriColorer>) -> Triangle
    {
    	let using_color_data : bool = (point_one.1.is_none() && point_two.1.is_none() && point_two.1.is_none());
    	
    	if (using_color_data)
    	{
    		assert_eq!(point_one.1.clone().unwrap().len(), point_two.1.clone().unwrap().len(), "All triangle point coloring data must be of equal length");
	    	assert_eq!(point_two.1.clone().unwrap().len(), point_three.1.clone().unwrap().len(), "All triangle point coloring data must be of equal length");	
    	}
    	
        let mut ret : Triangle = Triangle
        {
            points : [point_one.0, point_two.0, point_three.0],
            lowest_x : (None, None),
            highest_x : (None, None),
            lowest_y : (None, None),
            highest_y : (None, None),
            width : 0_f64,
            height : 0_f64,
            coloring_data : if (using_color_data) {Some([point_one.1.unwrap(), point_two.1.unwrap(), point_three.1.unwrap()])} else {None},
            colorer : colorer
        };
        ret.update_tri();
        return ret;
    }

    pub fn get_point(self : &Triangle, index : usize) -> Point
    {
        return Point
        {
            x : self.points[index].x,
            y : self.points[index].y,
        };
    }

    pub fn set_point(self : &mut Triangle, index : usize, point : Point)
    {
        self.points[index] = point;
        self.update_tri();
    }

    //A function for calculating the highest and lowest points for x and y.
    //Also updates triangle width and height
    fn update_tri(self : &mut Triangle) -> ()
    {
        //initial values are so first comparison will always be true
        let mut searcher_highest_x : f64 = std::f64::MIN;
        let mut searcher_lowest_x : f64 = std::f64::MAX;
        let mut searcher_highest_y : f64 = std::f64::MIN;
        let mut searcher_lowest_y : f64 = std::f64::MAX;

        for (point, point_idx) in self.points.iter().zip(0_u8..3_u8)
        {
            //---Xs---
            //Highest x
            if point.x > searcher_highest_x
            {
                self.highest_x.0 = Some(point_idx);
                searcher_highest_x = point.x;
            }
            else if approx_eq!(f64, point.x, searcher_highest_x)
            {
                self.highest_x.1 = Some(point_idx);
            }

            //Lowest x
            if point.x < searcher_lowest_x
            {
                self.lowest_x.0 = Some(point_idx);
                searcher_lowest_x = point.x;
            }
            else if approx_eq!(f64, point.x, searcher_lowest_x)
            {
                self.lowest_x.1 = Some(point_idx);
            }

            //---Ys---
            //Highest y
            if point.y > searcher_highest_y
            {
                self.highest_y.0 = Some(point_idx);
                searcher_highest_y = point.y;
            }
            else if approx_eq!(f64, point.y, searcher_highest_y)
            {
                self.highest_y.1 = Some(point_idx);
            }

            //Lowest y
            if point.y < searcher_lowest_y
            {
                self.lowest_y.0 = Some(point_idx);
                searcher_lowest_y = point.y;
            }
            else if approx_eq!(f64, point.y, searcher_lowest_y)
            {
                self.lowest_y.1 = Some(point_idx);
            }
        }

        self.width = f64::abs(self.points[self.highest_x.0.unwrap() as usize].x - self.points[self.lowest_x.0.unwrap() as usize].x);
        self.height = f64::abs(self.points[self.highest_y.0.unwrap() as usize].y - self.points[self.lowest_y.0.unwrap() as usize].y);
    }

    pub fn calc_weights(self : &Triangle, p : &Point) -> (f64, f64, f64)
	{
		let p_1 : Point = self.points[0];
		let p_2 : Point = self.points[1];
		let p_3 : Point = self.points[2];

		let (w_1, w_2, w_3) : (f64, f64, f64);
		
		let denom : f64 = (p_2.y - p_3.y) * (p_1.x - p_3.x) + (p_3.x - p_2.x) * (p_1.y - p_3.y);
		
		w_1 = ( (p_2.y - p_3.y) * (p.x - p_3.x) + (p_3.x - p_2.x) * (p.y - p_3.y) ) / denom;
		w_2 = ( (p_3.y - p_1.y) * (p.x - p_3.x) + (p_1.x - p_3.x) * (p.y - p_3.y) ) / denom;
		w_3 = 1_f64 - w_1 - w_2;

		(w_1, w_2, w_3)
	}

}
