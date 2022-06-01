pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if key < array[0] || key > array[array.len() - 1]{
        return None;
    }
    find_number(array, key, array.len() - 1, 0)
}

fn find_number(array: &[i32], key: i32, top_index: usize, bottom_index: usize) -> Option<usize>{

    if array[top_index / 2] == key{
        return Some((top_index / 2) as usize);
    }

    if array[top_index] == key{
        return Some((top_index) as usize);
    }

    if array[bottom_index / 2] == key{
        return Some((bottom_index / 2) as usize);
    }

    if key < array[top_index / 2] && key > array[bottom_index]{
        return find_number(array, key, top_index / 2, bottom_index);
    }

    if key > array[top_index / 2] && key < array[top_index]{
        return  find_number(array, key, top_index, top_index/2)
    }

    None
}
