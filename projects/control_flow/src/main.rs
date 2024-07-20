fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // the values that have the potential to be results from each arm of the if must be the same type;
    //5, 6 and not six

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number!=0 {
        println!("Non zero number");
    }

    if number % 4 == 0 {
        println!("Number divisible by 4");
    } else if number % 2 == 0 {
        println!("Number divisible by 2");
    } else if number % 3 == 0 {
        println!("Number divisible by 3");
    } else {
        println!("number not divisible by 2, 3, 4")
    }


    
}
