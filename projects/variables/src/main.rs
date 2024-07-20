fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    let ex_string = "   ";
    let ex_string = ex_string.len();

    //let mut ex_string = "   ";
    //ex_string = ex_string.len();
    //The error says we’re not allowed to mutate a variable’s type


    let sum = 5 + 10;
    let difference = 95.6 - 4.1;
    let product = 4 * 32;
    let quotient = 56.7/32.2;
    let _truncated = -5/3;//Results in -1
    let remainder = 43%5;

    println!("sum: {sum} \n difference: {difference} \n product: {product} \n quotient: {quotient} \n remainder: {remainder} \n"); //truncated: {truncated}");


    
}
