use std::cmp;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut first = 0;
    let mut last = array.len() - 1;
    while first <= last {
        let pos = ((last + first) / 2) as usize;
        match array[pos].cmp(&key) {
            cmp::Ordering::Equal => return Some(pos),
            cmp::Ordering::Less => first = pos + 1,
            cmp::Ordering::Greater => match pos {
                0 => return None,
                _ => last = pos - 1,
            },
        }
    }
    None
}
