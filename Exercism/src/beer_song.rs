const verse1: &str = " bottles of beer on the wall, ";
const verse2: &str = " bottles of beer.\n";
const verse3: &str = "Take one down and pass it around, ";
const verse4: &str = " bottles of beer on the wall.\n";

pub fn verse(n: u32) -> String {
    let mut res = String::new();
    res.push_str(&*n.to_string());
    res.push_str(verse1);
    res.push_str(&*n.to_string());
    res.push_str(verse2);
    res.push_str(verse3);
    res.push_str(&*(n-1).to_string());
    res.push_str(verse4);
    res
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res= "".to_string();

    for i in start..end{
        res.push_str(&*verse(i));
    }
    res
}
