use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("yellow".to_string(), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn strings() {
    let mut s = "text".to_string();
    let s1 = "jamo".to_string();
    s.push_str(&s1);
}

fn vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{} ", i);
    }

    let mut v1 = vec![3, 7, 8];

    for i in &mut v1 {
        *i += 50;
    }
}
