use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();
    h.iter().for_each(|m| {
        let num = *m.0;
        for c in m.1 {
            let lower_char = c.to_lowercase().to_string().chars().nth(0).unwrap();
            result.insert(lower_char, num);
        }
    });
    result
}
