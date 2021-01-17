use std::collections::vec_deque;

pub struct PascalsTriangle {
    row_count: u32,
    _secret: (),
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count: row_count,
            _secret: (),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = vec![];
        for _ in 1..self.row_count + 1 {
            let row = row(result.last());
            result.push(row);
        }
        result
    }
}
fn row(last_row: Option<&Vec<u32>>) -> Vec<u32> {
    let mut row: Vec<u32> = vec![];
    row.push(1);
    if last_row.is_none() {
        return row;
    }
    
    for elements in last_row.unwrap().windows(2) {
        row.push(elements.iter().sum::<u32>());
    }
    row.push(1);
    row
}
