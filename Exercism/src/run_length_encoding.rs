pub fn encode(source: &str) -> String {
    let mut res: String = "".to_string();

    if source.len() == 0{
        return res;
    }

    let mut last_c = source.as_bytes()[0] as char;
    let mut counter = 1;

    for i in 1..source.len(){
        let c = source.as_bytes()[i] as char;

        if c == last_c {
            counter += 1;
            continue;
        }

        if counter > 1 {
            res.push_str(&counter.to_string());
        }

        res.push(last_c);
        last_c = c;
        counter = 1;
    }

    if counter > 1 {
        res.push_str(&counter.to_string());
    }

    res.push(last_c);

    res
}

pub fn decode(source: &str) -> String {
    let mut res: String = "".to_string();

    let mut count = 0;
    let mut count_as_string: String = "".to_string();

    for c in source.chars(){

        if c.is_numeric(){
            count_as_string.push(c);
            continue;
        }

        count = count_as_string.parse::<i32>().unwrap_or(1);
        count_as_string = "".to_string();

        if count == 1{
            res.push(c);
            continue;
        }

        for i in 0..count{
            res.push(c);
        }
    }

    res
}
