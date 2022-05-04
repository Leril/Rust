fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s2);

    let mut s = String::from("bye ");

    println!("{}", calculate_size(&s));
    change(&mut s);
    println!("{}", s);

    let mut s1 = &mut s;
    let mut _s2 = &mut s1;
    _s2.push_str("str");
    println!("{}", s2);
    let sentence = String::from("first and second words");
    let word = first_word(&sentence);
    let sword = second_word(&sentence);
    println!("first word is: {}", word);
    println!("second word is: {}", sword);
}

fn second_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    let mut j = 0;

    for(i, &item) in bytes.iter().enumerate(){
      if j == 0 && item == b' '{
          j = i;
      }else if item == b' '{
          return &s[j..i];
      }
    }
    &s
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s
}

fn calculate_size(s: &String) -> usize{
    s.len()
}

fn change(s: &mut String){
    s.push_str("bye");
}