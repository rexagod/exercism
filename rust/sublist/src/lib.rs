#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() <= _second_list.len() && is_sublist(_first_list, _second_list) {
        if _first_list.len() == _second_list.len() {
            Comparison::Equal
        } else {
            Comparison::Sublist
        }
    } else if _second_list.len() < _first_list.len() &&  is_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _first_list.len() == 0 {
        return true
    }
    for j in 0 .. (_second_list.len() - _first_list.len() + 1) {
        if j + _first_list.len() <= _second_list.len() && _second_list[j..(j+_first_list.len())] == *_first_list {
            return true
        }
    }
    false
}
