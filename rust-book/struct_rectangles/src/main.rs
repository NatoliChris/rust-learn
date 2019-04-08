
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rect) -> bool {
        self.height > rect2.height && self.width > rect2.height
    }

    // Associated function
    fn square(size: u32) -> Rect {
        Rect { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rect { width: 30, height: 50 };
    let rect2 = Rect { width: 20, height: 20 };
    let rect3 = Rect::square(20);

    println!( "The area of the rect: {}",
        rect1.area()
    );

    println!("Rect1 can hold Rect2? {}", rect1.can_hold(&rect2));
    println!("Rect1 can hold Rect3? {}", rect1.can_hold(&rect3));

}
