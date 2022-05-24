use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res:HashSet<&str> = HashSet::new();
    let mut map: HashMap<Vec<char>, u32> = HashMap::new();

    for c in word.chars(){
        let x = map.entry(c.to_lowercase().collect()).or_insert(0);
        *x += 1;
        println!("{}", c.to_lowercase());
    }

    let b = word.to_lowercase();
    for possible_word in possible_anagrams{
        let a = possible_word.to_lowercase();
        if a.eq(&b){
            continue;
        }

        let mut aux_map = map.clone();
        let mut all = true;

        for c in possible_word.chars(){
            println!("{}", c.to_lowercase());
            let mut x = *aux_map.entry(c.to_lowercase().collect()).or_insert(0);

            if x == 0{
                all = false;
                break;
            }

            x -= 1;

            println!("num: {}", x);

            if x == 0{
                let vc:Vec<char> = c.to_lowercase().collect();
                aux_map.remove(&vc);
            }else{
                aux_map.insert(c.to_lowercase().collect(), x);
            }
        }

        println!("{:?}", aux_map.keys().into_iter());

        if all && aux_map.is_empty(){
            res.insert(possible_word);
        }
        println!("all: {}, is_empty: {}", all, aux_map.is_empty());
    }
    res
}
