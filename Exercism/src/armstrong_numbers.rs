pub fn is_armstrong_number(num: u32) -> bool {
    let mut v:Vec<u32> = Vec::new();
    let mut num1 = num;

    while num1 > 0{
        v.push(num1 % 10);
        num1 = num1 / 10;
    }

    let mut sum = 0;

    for n in 0..v.len(){
        sum += v[n].pow(v.len() as u32)
    }

    num == sum
}
