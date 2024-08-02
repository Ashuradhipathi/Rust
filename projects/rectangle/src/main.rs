#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn main() {
    //let scale = 2;
    let rect1 = Rectangle{
        //width:dbg!(scale*30),
        width:30,
        height:50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    add();
    println!(
        "The area of rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("Width of rectangle is greater than 0")
    }

    //println!("{rect1:#?}");
    //dbg!(&rect1);
}

impl Rectangle{

fn width(&self) -> bool {
    self.width > 0
}
fn area(&self) -> u32 {
    self.width * self.height
}
fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
}

fn square(size: u32) -> Self {
    Self {
        width: size,
        height: size,
    }
}
}

fn add() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

}