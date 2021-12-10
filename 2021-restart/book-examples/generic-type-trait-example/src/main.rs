use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    // The ADD trait is:
    //
    // trait Add<Rhs=Self> {
    //     type Output;
    //
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }
}

fn main() {
    let a = Point { x: 1, y: 1 };
    let b = Point { x: 1, y: 1 };

    println!("{:?}", a + b);
}
