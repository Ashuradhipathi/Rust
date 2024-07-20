fn main() {
    //loop{
    //    println!("Hello, world!");
    //}
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!(" The result is {result}");
    multi_loop();
    while_loop();
    for_loop();
}


fn multi_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count {count}");

        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
        }

        count += 1;
    }
    println!("End Count = {count}");
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number-=1;
    }

    println!("LAUNCH!!");

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < a.len() {
        println!(" {}",a[index]);
        index+=1;
    }

}


fn for_loop() {
    let a = [10,20,30,40,50];

    for element in a {
        println!("The value: {element}");
    }

    for number in (1..a.len()).rev(){
        println!("The value: {}",a[number]);
    }
}