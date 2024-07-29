struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64,
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user2 = User{
        active:true,
        username:String::from("testuser2"),
        email:String::from("testuser@gmail.com"),
        sign_in_count:1,
    };
    println!("{}",user2.email);
    user2.email = String::from("testuser2@gmail.com");
    println!("{}",user2.email);

    let mut user3 = User{
        email:String::from("testuser@gmail.com"),
        ..user2
    };
    //println!("{}",user2.username);
    println!("{}",user2.email);

    let black = Color(0,0,0);
    
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;
}


fn build_user(email:String, username:String) -> User{
    User{
        active:true,
        username,
        email,
        sign_in_count:1,
    }
}