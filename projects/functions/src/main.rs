fn main() {
    println!("Hello, world!");
    print_labeled_measurements(5,'h');
    let x = increment(5);
    let exp = five();
    println!("{exp}");
    println!("\n {x}");
}

fn print_labeled_measurements( x:i32, unit_label: char) {
    println!("Measurement value : {x} {unit_label}");
}

fn increment(x:i32)->i32 {
    let y = {
        x+1
    };

    println!("y : {y}");
    y
}


fn five() -> i32 {
    5
}