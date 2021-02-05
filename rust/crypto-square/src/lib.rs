pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let result = normalize(input).and_then(|input| {
        let column = calculate_column_size(&input);
        let init: Vec<Vec<char>> = vec![];
        Ok(input
            .chars()
            .collect::<Vec<char>>()
            .chunks(column as usize)
            .fold(init, |mut acc, chunk| {
                (0..column).for_each(|jdx| {
                    let c = match jdx < chunk.len() {
                        true => chunk[jdx],
                        _ => ' ',
                    };
                    if let Some(row) = acc.get_mut(jdx) {
                        row.push(c);
                    } else {
                        acc.push(vec![c]);
                    }
                });
                acc
            })
            .iter()
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" "))
    });
    result.unwrap()
}

fn normalize(input: &str) -> Result<String, ()> {
    Ok(input
        .replace(|c: char| !c.is_ascii_alphanumeric(), "")
        .to_ascii_lowercase())
}

fn calculate_column_size(input: &String) -> usize {
    let length = input.len();
    let tmp_size = (length as f32).sqrt() as usize;
    match length > tmp_size.pow(2) {
        true => tmp_size + 1,
        _ => tmp_size,
    }
}
