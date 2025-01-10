use float_cmp::approx_eq;

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
        let slope : f64 = (line_point_two.y - line_point_one.y) / (line_point_two.x - line_point_one.x);
        return self.y >= slope * (self.x - line_point_one.x) + line_point_one.y;
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
    //The *indexes* of which point has these traits
    //Having to constantly convert these to usizes for indexing is some shit
    //Represented as tuples becuase two points can share the same extreme
    //Option type used to represent the fact that each extreme can have just one point (and in most cases this is the case)
    pub lowest_x : (Option<u8>, Option<u8>),
    pub highest_x : (Option<u8>, Option<u8>),
    pub lowest_y : (Option<u8>, Option<u8>),
    pub highest_y : (Option<u8>, Option<u8>),
}

impl Triangle
{
    pub fn new(point_one : Point, point_two : Point, point_three : Point) -> Self
    {
        let mut ret : Self = Self
        {
            points : [point_one, point_two, point_three],
            lowest_x : (None, None),
            highest_x : (None, None),
            lowest_y : (None, None),
            highest_y : (None, None),

        };
        ret.calc_extremes();
        return ret;
    }

    pub fn get_point(self : &Self, index : usize) -> Point
    {
        return Point
        {
            x : self.points[index].x,
            y : self.points[index].y,
        };
    }

    pub fn set_point(self : &mut Self, index : usize, point : Point)
    {
        self.points[index] = point;
        self.calc_extremes();
    }

    //A function for calculating the highest and lowest points for x and y.
    fn calc_extremes(self : &mut Self) -> ()
    {
        //initial values so first comparison will always be true
        let mut searcher_highest_x : f64 = std::f64::MIN;
        let mut searcher_lowest_x : f64 = std::f64::MAX;
        let mut searcher_highest_y : f64 = std::f64::MIN;
        let mut searcher_lowest_y : f64 = std::f64::MAX;

        for (point, point_idx) in self.points.iter().zip(0_u8..0_4u8)
        {
            //---Xs---
            //Highest x
            if point.x > searcher_highest_x
            {
                self.highest_x.0 = Some(point_idx);
                searcher_highest_x = point.x;
            }
            if approx_eq!(f64, point.x, searcher_highest_x)
            {
                self.highest_x.1 = Some(point_idx);
            }

            //Lowest x
            if point.x < searcher_lowest_x
            {
                self.lowest_x.0 = Some(point_idx);
                searcher_lowest_x = point.x;
            }
            if approx_eq!(f64, point.x, searcher_lowest_x)
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
            if approx_eq!(f64, point.y, searcher_highest_y)
            {
                self.highest_y.1 = Some(point_idx);
            }

            //Lowest y
            if point.y < searcher_lowest_y
            {
                self.lowest_y.0 = Some(point_idx);
                searcher_lowest_y = point.y;
            }
            if approx_eq!(f64, point.y, searcher_lowest_y)
            {
                self.lowest_y.1 = Some(point_idx);
            }
        }
    }
}