fn main() {
    println!("Hello, world!");
    another_function(15, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("Value of y: {}", y);
    println!("Value of returning function: {}", returning_function());
    println!("Value of adding function: {}", adding_function(5));
}

fn adding_function(x: u32) -> u32{
    return x + 1;
}

fn returning_function() -> u32 {
    15
}

fn another_function(x: u32, letter: char){
    println!("Hello from another function");
    println!("Value passed to another function: {}, {}", x, letter);
}
