pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for i in 0..minefield.len(){
        let mut string_to_add = "".to_string();
        for j in 0..minefield[0].len(){

            if minefield[i].as_bytes()[j] == b'*'{
                string_to_add.push("*".parse().unwrap());
                continue;
            }

            let mut counter = 0;

            for x in 0..3{
                for y in 0..3{
                    if x == 1 && y == 1{
                        continue;
                    }

                    let x1:i32 = ((i + x) as i32) - 1;
                    let y1:i32 = ((j + y) as i32) - 1;

                    if is_in_range(minefield.len(), x1)
                        && is_in_range(minefield[0].len(), y1){
                        if minefield[(x1 as usize)].as_bytes()[(y1 as usize)] == b'*'{
                            counter += 1;
                        }
                    }
                }
            }

            if counter == 0{
                string_to_add.push(" ".parse().unwrap());
            }else{
                string_to_add.push(char::from_digit(counter, 10).unwrap_or_default());
            }
        }
        res.push(string_to_add);
    }

    res
}

fn is_in_range(ar: usize, index: i32) -> bool{
    index < ar as i32 && index >= 0
}
