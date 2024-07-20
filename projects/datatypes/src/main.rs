fn main() {
    let t = true;
    let f: bool = false;

    println!("True {t}");
    println!("False {f}");


    let c = 'z';
    let z: char =  'ℤ';
    let heart_eyes_cat = '😻';

    println!("c: {c} \n z: {z} \n heart_eyes_cat: {heart_eyes_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (k, l, m) = tup;

    println!("k: {k} \n l: {l} \n m: {m}");


    let comp: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = comp.0;
    let six_point_four = comp.1;
    let one = comp.2;

    println!("five_hundred: {five_hundred} \n six_point_four: {six_point_four} \n one: {one}");

    let a = [1,2,3,4,5];



    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];


    let arr: [i32;5] = [1,2,3,4,5];

    let ar = [3;5]; //[value;length]

    let first = a[0];
    println!("first: {first} \n ");
}
