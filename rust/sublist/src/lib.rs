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
    if first_list.len() == second_list.len() {
        if first_list.eq(&second_list) {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if first_list.len() < second_list.len() {
        let subtract = second_list.len() - first_list.len();
        for i in 0..subtract + 1 {
            if second_list
                .iter()
                .skip(i)
                .take(first_list.len())
                .cloned()
                .collect::<Vec<_>>()
                .eq(&first_list)
            {
                return Comparison::Sublist;
            }
        }
        Comparison::Unequal
    } else {
        let subtract = first_list.len() - second_list.len();
        for i in 0..subtract + 1 {
            if first_list
                .iter()
                .skip(i)
                .take(second_list.len())
                .cloned()
                .collect::<Vec<_>>()
                .eq(&second_list)
            {
                return Comparison::Superlist;
            }
        }
        Comparison::Unequal
    }
}
