pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point {
            x: x,
            y: y
        }
    }
}

pub struct Rect<T> {
    pub loc: Point<T>,
    pub width: T,
    pub height: T
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Rect {
            loc: Point::new(x,y),
            width: w,
            height: h
        }
    }
}

pub struct Circle<T> {
    pub loc: Point<T>,
    pub size: T,
}

impl<T> Circle<T> {
    pub fn new(x: T, y: T, size: T) -> Self {
        Circle {
            loc: Point::new(x,y),
            size: size
        }
    }
}