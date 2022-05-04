use std::thread::sleep;
use std::time::Duration;

fn main() {
    ifs();
    loops();
    returning_loop();
    for_loops();
}

fn for_loops(){
    let col = [1, 3, 5, 7, 9];
    let mut index = 0;

    while index < col.len() {
        println!("The value is: {}", col[index]);

        index += 1;
    }

    for element in col{
        println!("The value from for is: {}", element);
    }

    for number in (1..6).rev(){
        println!("{}!", number);
        sleep(Duration::from_secs(1));
    }

    println!("LIFT OFF !!!");
}

fn returning_loop(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result)
}

fn loops(){
    let mut count = 0;

    'outer: loop {
        println!("count is: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining is: {}", remaining);
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'outer;
            }

            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {}", count)
}

fn ifs(){
    let num = 4;

    if num < 5 {
        println!("the condition was true");
    }else {
        println!("the condition was false");
    }

    let number = 12;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num = if condition {5} else {0};

    println!("the value depending on condition is: {}", num);
}
