/*fn main() {
    println!("Hello, world!");

    int_copy_check();
    string_copy_working();
    //let st = "world";
    //println!("{s} : {st}");
    //let s = "hello";
    //let mut str = String::from("Hey");

    {
        //#let s = "hello";
        // #let mut str = String::from("Hey");
        //println!("{s} : {st}"); 
        //println!("{str}");
    }
    //println!("{} : {}",s,st); 
    //let x = 1;
    //let y = 2;

    //println!("{}",x+y);

    
    //str.push_str(",Hii");
    //println!("{str}");

}

fn int_copy_check(){
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

}
fn string_copy_working(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
//     let s2 = s1;
//|              -- value moved here
//31 |
//32 |     println!("{s1}, world!");
//   |               ^^^^ value borrowed here after move
    println!("s1 = {s1},s2 = {s2}");

}*/
/*fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
 */
 /*fn main() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}*/
/*fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}*/
/*fn main(){
    let mut s1 = String::from("Heyy");

    let len = calculate_length(&mut s1);

    println!("{}",len);
    {
        let r1 = &mut s1;
        println!("{r1}");
    }


    let r3 = &s1;
    let r4 = &s1;
    println!("{r3}");
    println!("{r4}");

    println!("{r3}");
    println!("{r4}");

    let r2 = &mut s1;

    println!("{r2}");
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", World!");
    s.len()
}*/
fn main(){
    let mut my_string = String::from("hello world");

  // `first_word` works on slices of `String`s, whether partial or whole
  let word = first_word(&my_string[0..6]);
  let word = first_word(&my_string[..]);
  // `first_word` also works on references to `String`s, which are equivalent
  // to whole slices of `String`s
  let word = first_word(&my_string);

  let my_string_literal = "hello world";

  // `first_word` works on slices of string literals, whether partial or whole
  let word = first_word(&my_string_literal[0..6]);
  let word = first_word(&my_string_literal[..]);

  // Because string literals *are* string slices already,
  // this works too, without the slice syntax!
  let word = first_word(my_string_literal);

    println!("{word}");


    let a = [1,2,3,4,5];

    let slice = &a[1..4];

assert_eq!(slice, &[2,3,4]);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}