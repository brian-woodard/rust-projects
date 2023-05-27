
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };

    println!("The area of the rectangle is: {}",
             area(&rect1));

    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
