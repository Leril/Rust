use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    numeric_types();
    shadowing();
    tuples();
    arrays();
}

fn arrays(){
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn tuples(){
    let tup: (i32, f64, u8, bool) = (500, 6.3, 18, true);
    println!("tuple fourth: {}", tup.3);

    let (x, y, z, w) = tup;

    println!("individually tuples: {}, {}, {}, {}", x, y, z, w)
}

fn shadowing () {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6;

    {
        let x =  x + 7;
        println!("the value of x in inner scope is {}", x);
    }
    println!("The value of x is: {}", x);
}

fn numeric_types() {
    let sum = 5 + 10;

    let diff = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7/32.2;
    let floored = 2/3;

    let remainder = 43 % 5;

    print!("sum is: {}\n", sum);
    print!("diff is: {}\n", diff);
    print!("product is: {}\n", product);
    print!("quotient is: {}\n", quotient);
    print!("floored is: {}\n", floored);
    print!("remainder is: {}\n", remainder);
}
