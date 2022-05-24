/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    if code.len() < 2{
        return false;
    }

    let mut nums:Vec<u32> = Vec::new();

    for c in code.chars().rev(){
        if !c.is_numeric(){
            return false
        }
        nums.push(c.to_digit(10).unwrap_or(0));
    }

    for i in 0..nums.len(){
        if i % 2 == 1{
            let mut num = nums[i];
            num *= 2;
            if num > 9{
                num -= 9;
            }
            nums[i] = num;
        }
    }

    let sum:u32 = nums.iter().sum();
    sum % 10 == 0
}
