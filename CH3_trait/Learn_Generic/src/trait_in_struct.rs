use std::fmt::{self,Display};

// #[derive(Debug)]
pub struct Point1<T> {
    pub x: T,
    pub y: T,
}

impl Display for Point1<i32>{
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Point2<T,U> {
    x: T,
    y: U,
}

// ---trait in methods---

pub struct Point3<T> {
    x:T, 
    y:T,
}

#[allow(dead_code)]
impl <T> Point3<T> {
    pub fn new(x:T, y:T) -> Point3<T> {
        Point3 {x,y}
    }
    pub fn get_x(&self) -> &T{
        &self.x
    }
    pub fn get_y(&self) -> &T{
        &self.y
    }
}


#[allow(dead_code)]
impl <T,U> Point2<T,U> {
    pub fn new(x:T, y:U) -> Point2<T,U> {
        Point2 {x,y}
    }
    pub fn get_x(&self) -> &T{
        &self.x
    }
    pub fn get_y(&self) -> &U{
        &self.y
    }
}