
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let sq = Rectangle::square(25);

    println!("area {}", rect1.area());
    println!("area {}", rect2.area());
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("area {}", sq.area());
}
