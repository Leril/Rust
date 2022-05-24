// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map:HashMap<&str, u32> = HashMap::new();

    for word in magazine{
        if map.contains_key(word){
            let mut num = map.get(word).unwrap();
            let num1 = num + 1;
            map.insert(word, num1);
        }else{
            map.insert(word, 1);
        }
    }

    for word in note{
        if map.contains_key(word){
            let mut num = map.get(word).unwrap();
            if *num == 0{
                return false
            }
            let num1 = num - 1;
            map.insert(word, num1);
        }else{
            return false;
        }
    }
    true
}
