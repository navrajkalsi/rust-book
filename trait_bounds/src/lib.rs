pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // here components is a vector of trait objects that implement the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // NOTE one Screen instance can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>.
            // this would not be possible if we had used a trait bound, like Vec<T : Draw>
            // with trait bound, rust will lock in the first type that is used with the vec and we
            // cannot use any other type, even if it implements the Draw trait
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
