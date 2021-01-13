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

    if _first_list.len() <= _second_list.len() {
        if _first_list.eq(_second_list) {
            Comparison::Equal
        } else if is_sublist(_first_list, _second_list) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else {
        if is_sublist(_second_list, _first_list) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    }
}

fn is_sublist<T: PartialEq>(smaller_list: &[T], larger_list: &[T]) -> bool {
    larger_list
        .windows(smaller_list.len())
        .find(|windows| windows.eq(&smaller_list))
        .is_some()
}
