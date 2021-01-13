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

    match first_list.len().cmp(&second_list.len()) {
        std::cmp::Ordering::Equal => {
            if first_list.eq(&second_list) {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        std::cmp::Ordering::Less => {
            if is_sublist(first_list, second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        std::cmp::Ordering::Greater => {
            if is_sublist(second_list, first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn is_sublist<T: PartialEq>(smaller_list: Vec<&T>, larger_list: Vec<&T>) -> bool {
    let subtract = larger_list.len() - smaller_list.len();
    for i in 0..subtract + 1 {
        if larger_list
            .iter()
            .skip(i)
            .cloned()
            .take(smaller_list.len())
            .collect::<Vec<_>>()
            .eq(&smaller_list)
        {
            return true;
        }
    }
    false
}
