#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// methods have self as the first parameter
impl Rectangle {
    // &self is shorthand for 'self : &Self'
    // here Self is the struct,
    // can also be written as 'self : &Rectangle'
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {
    // impl blocks can be separated
    // Self is what appears after 'impl' at the start of this block
    fn square(edge: u32) -> Self {
        Self {
            width: edge,
            height: edge,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("area_parameters: {}", area_parameters(width1, height1));

    let rect1 = (width1, height1);
    println!("area_tuple: {}", area_tuple(rect1));

    let rect2 = Rectangle {
        width: width1,
        height: height1,
    };
    println!("rect2: {rect2:?}");
    println!("pretty_rect2: {rect2:#?}");
    println!("area_struct: {}", area_struct(&rect2));

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(width1 * scale),
        height: height1,
    };
    dbg!(&rect3); // rect3 can still be used after this, cause ownership was not passed

    println!("area_method: {}", rect3.area());

    let rect4 = Rectangle {
        width: 20,
        height: 50,
    };

    println!("can_hold: {}", rect3.can_hold(&rect4));

    let square = dbg!(Rectangle::square(10));
    println!("square_edge: {}", square.width);
}

fn area_parameters(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
