pub fn reverse(input: &str) -> String {
    let mut rev = String::new();

    for c in input.chars().rev(){
        rev.push(c)
    }

    rev
}
