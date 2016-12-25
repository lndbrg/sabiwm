/// A simple rectangle. Not much to talk about.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Gets the x coordinate of the right hand border
    fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    /// Gets the y coordinate of the bottom hand border
    fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }

    /// Creates a new rectangle from the given coordinates (upper left corner + size)
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::Rectangle;
    /// let rectangle = Rectangle::new(10, 10, 12, 13);
    /// ```
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    /// Checks if the given coordinates are within the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// # use sabiwm::core::Rectangle;
    /// let rectangle = Rectangle::new(10, 10, 20, 10);
    /// assert_eq!(true, rectangle.is_inside(12, 15));
    /// assert_eq!(false, rectangle.is_inside(12, 21));
    /// ```
    ///
    /// # Arguments 
    /// `x` - x coordinate of point
    /// `y` - y coordinate of point
    ///
    /// # Return value
    /// `true` if point is inside, `false` otherwise
    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        let horizontal = x >= self.x && x <= self.right();
        let vertical = y >= self.y && y <= self.bottom();

        horizontal && vertical
    }

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        !(other.x >= self.right() || other.right() <= self.x || other.y >= self.bottom() ||
          other.bottom() <= self.y)
    }
}
