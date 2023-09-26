fn main() {
    let rect1: Rectangle = Rectangle::new(3.0, 4.0);
    let mut rect2: Rectangle = Rectangle::new(1.2, 4.5);
    let rect3: Rectangle = Rectangle::new(6.7, 9.8);
    let mut rect4: Rectangle = Rectangle::new(0.5, 0.2);

    let mut trash: Rectangle = Rectangle::new(10., 10.);

    // .area() borrows each rectangle immutably
    println!("Area of rect1: {}", rect1.area());
    println!("Area of rect2: {}", rect2.area());
    println!("Area of rect3: {}", rect3.area());
    println!("Area of rect4: {}", rect4.area());

    // .max_TAKE_other() takes ownership of trash and then
    // deallocates the Rectangle on the heap once it returns.
    let max_rect_1_trash: Rectangle = rect1.max_TAKE_other(trash);

    println!("This print still works for rect1: {}", rect1.area());
    // println!("But this print does not work for trash: {}", trash.area());
    println!("This print works for max_rect_1_trash: {}", max_rect_1_trash.area());

    trash = Rectangle::new(5.0, 5.0);

    let max_rect_2_trash: Rectangle = rect2.max_BORROW_other(&trash);

    println!("This print still works for rect 2: {}", rect2.area());
    println!("And this print works for trash: {}", trash.area());
    println!("This print works for max_rect_2_trash: {}", max_rect_2_trash.area());

    // This call doesn't work for either rect1 or rect3, because they are immutable:
    //
    // rect1.set_width(7.0);
    // rect3.set_height(-400.98);

    // But this call does work for rect2, rect4, and trash:
    rect2.set_height(9.0);
    rect4.set_width(22.4);
    trash.set_height(2.3);
    trash.set_width(44.6);

    println!("No whammies: {}", rect2.area());
    println!("No whammies: {}", rect4.area());
    println!("No whammies: {}", trash.area());

    // If we make an immutable reference to a mutable variable,
    // we still get bounced when we take ownership and try to
    // mutate through the reference
    //
    // let dummy: &mut Rectangle = &mut trash;
    // dummy.set_width(22.0);
    // dummy.height = 55.1; //implicit dereference. Same as (*dummy).height
    // println!("dummy: {} x {}", dummy.get_width(), dummy.get_height());


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

    //
    fn can_hold(&self, other: Rectangle) -> bool {
        other.get_width() <= self.width && other.get_height() <= self.height
    }

    fn max_TAKE_other(&self, other: Rectangle) -> Rectangle {
        Rectangle { 
            height: self.height.max(other.get_height()), 
            width: self.width.max(other.get_width())
        }
    }

    fn max_BORROW_other(&self, other: &Rectangle) -> Rectangle {
        Rectangle { 
            height: self.height.max(other.get_height()), 
            width: self.width.max(other.get_width())
         }
    }
}