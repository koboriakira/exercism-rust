use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Ord + Display + Debug>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    if _first_list.is_empty() {
        if _second_list.is_empty() {
            return Comparison::Equal;
        } else {
            return Comparison::Sublist;
        }
    } else if _second_list.is_empty() {
        return Comparison::Superlist;
    }

    let first_list: Vec<_> = _first_list.iter().collect();
    let second_list: Vec<_> = _second_list.iter().collect();

    if first_list.len() <= second_list.len() {
        if first_list.eq(&second_list) {
            Comparison::Equal
        } else if is_sublist(first_list, second_list) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else {
        if is_sublist(second_list, first_list) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    }
}

fn is_sublist<T: PartialEq + Debug>(smaller_list: Vec<&T>, larger_list: Vec<&T>) -> bool {
    larger_list
        .windows(smaller_list.len())
        .find(|windows| windows.eq(&smaller_list))
        .is_some()
}
