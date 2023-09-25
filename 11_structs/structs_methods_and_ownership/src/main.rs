fn main() {
    let rect = Rectangle::new(3.0, 4.0);
}

struct Rectangle {
    height: f32,
    width: f32,
}

impl Rectangle {
    fn new(height: f32, width: f32) -> Self {
        Rectangle { 
            height: height, 
            width: width
        }
    }
    fn area(&self) -> f32 {
        self.height * self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.get_width() <= self.width && other.get_height() <= self.height
    }

    fn max(&self, other: &Rectangle) -> Rectangle {
        Rectangle { 
            height: self.height.max(other.get_height()), 
            width: self.width.max(other.get_width())
        }
    }
}