#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut res = Comparison::Equal;

    if is_equal(_first_list, _second_list){
        res = Comparison::Equal;
    }else if is_super_list(_first_list, _second_list){
        res = Comparison::Superlist
    } else if is_super_list(_second_list, _first_list){
        res = Comparison::Sublist
    }else {
        res = Comparison::Unequal
    }
    res
}

fn is_equal<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool{

    if _first_list.is_empty() && _second_list.is_empty(){
        return true;
    }

    if _first_list.is_empty()  || _first_list.len() != _second_list.len(){
        return false;
    }

    for i in 0.._first_list.len() {
        if _second_list.len() <= i{
            return false;
        }
        if _first_list[i] != _second_list[i]{
            return false;
        }
    }
    true
}

fn is_super_list<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool{

    if _first_list.is_empty(){
        return false;
    }

    if _second_list.is_empty(){
        return true;
    }


    for i in 0.._first_list.len(){
        if _first_list[i] == _second_list[0]{
            for j in 1.._second_list.len(){

                if _first_list.len() <= i + j{
                    return false;
                }

                if _first_list[i + j] == _second_list[j]{
                    if j == _second_list.len() - 1{
                        return true;
                    }
                    continue;
                }
                break
            }
        }
    }
    false
}
