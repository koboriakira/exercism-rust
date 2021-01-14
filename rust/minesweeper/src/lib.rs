struct Box {
    vertical: Vertical,
    horizontal: Horizontal,
    _secret: (),
}

impl Box {
    fn from(i: usize, j: usize, minefield: &Vec<Vec<char>>) -> Self {
        let vertical = match (i, minefield.len(), i == minefield.len() - 1) {
            (_, 1, _) => Vertical::OneRow,
            (0, _, _) => Vertical::Top,
            (_, _, true) => Vertical::Bottom,
            _ => Vertical::Other,
        };
        let horizontal = match (j, minefield[0].len(), j == minefield[0].len() - 1) {
            (_, 1, _) => Horizontal::OneColumn,
            (0, _, _) => Horizontal::Left,
            (_, _, true) => Horizontal::Right,
            _ => Horizontal::Other,
        };
        Box {
            vertical: vertical,
            horizontal: horizontal,
            _secret: (),
        }
    }

    fn generate_checkable_boxes(&self, i: usize, j: usize) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = Vec::new();

        match (self.vertical, self.horizontal) {
            (Vertical::Top, Horizontal::Left) => {
                result.push([i, j + 1].to_vec());
                result.push([i + 1, j].to_vec());
                result.push([i + 1, j + 1].to_vec());
            }
            (Vertical::Top, Horizontal::Right) => {
                result.push([i, j - 1].to_vec());
                result.push([i + 1, j - 1].to_vec());
                result.push([i + 1, j].to_vec());
            }
            (Vertical::Top, Horizontal::Other) => {
                result.push([i, j - 1].to_vec());
                result.push([i, j + 1].to_vec());
                result.push([i + 1, j - 1].to_vec());
                result.push([i + 1, j].to_vec());
                result.push([i + 1, j + 1].to_vec());
            }
            (Vertical::Top, Horizontal::OneColumn) => {
                result.push([i + 1, j].to_vec());
            }
            (Vertical::Bottom, Horizontal::Left) => {
                result.push([i - 1, j].to_vec());
                result.push([i - 1, j + 1].to_vec());
                result.push([i, j + 1].to_vec());
            }
            (Vertical::Bottom, Horizontal::Right) => {
                result.push([i - 1, j].to_vec());
                result.push([i - 1, j - 1].to_vec());
                result.push([i, j - 1].to_vec());
            }
            (Vertical::Bottom, Horizontal::Other) => {
                result.push([i - 1, j].to_vec());
                result.push([i - 1, j + 1].to_vec());
                result.push([i - 1, j - 1].to_vec());
                result.push([i, j + 1].to_vec());
                result.push([i, j - 1].to_vec());
            }
            (Vertical::Bottom, Horizontal::OneColumn) => {
                result.push([i - 1, j].to_vec());
            }
            (Vertical::OneRow, Horizontal::Left) => {
                result.push([i, j + 1].to_vec());
            }
            (Vertical::OneRow, Horizontal::Right) => {
                result.push([i, j - 1].to_vec());
            }
            (Vertical::OneRow, Horizontal::OneColumn) => {}
            (Vertical::OneRow, Horizontal::Other) => {
                result.push([i, j + 1].to_vec());
                result.push([i, j - 1].to_vec());
            }
            (Vertical::Other, Horizontal::Left) => {
                result.push([i - 1, j].to_vec());
                result.push([i - 1, j + 1].to_vec());
                result.push([i, j + 1].to_vec());
                result.push([i + 1, j].to_vec());
                result.push([i + 1, j + 1].to_vec());
            }
            (Vertical::Other, Horizontal::Right) => {
                result.push([i - 1, j].to_vec());
                result.push([i - 1, j - 1].to_vec());
                result.push([i, j - 1].to_vec());
                result.push([i + 1, j].to_vec());
                result.push([i + 1, j - 1].to_vec());
            }
            (Vertical::Other, Horizontal::OneColumn) => {
                result.push([i - 1, j].to_vec());
                result.push([i + 1, j].to_vec());
            }
            (Vertical::Other, Horizontal::Other) => {
                result.push([i - 1, j - 1].to_vec());
                result.push([i - 1, j].to_vec());
                result.push([i - 1, j + 1].to_vec());
                result.push([i, j - 1].to_vec());
                result.push([i, j + 1].to_vec());
                result.push([i + 1, j - 1].to_vec());
                result.push([i + 1, j].to_vec());
                result.push([i + 1, j + 1].to_vec());
            }
        }
        result
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Vertical {
    Top,
    Bottom,
    OneRow,
    Other,
}

#[derive(PartialEq, Clone, Copy)]
enum Horizontal {
    Left,
    Right,
    OneColumn,
    Other,
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let char_minefield: Vec<Vec<char>> = minefield
        .iter()
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect();
    (0..char_minefield.len())
        .map(|i| {
            (0..char_minefield[i].len())
                .map(|j| count_mines(i, j, &char_minefield))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}

fn count_mines(i: usize, j: usize, minefield: &Vec<Vec<char>>) -> String {
    if minefield[i][j] == '*' {
        return "*".to_string();
    }

    let mine_box = Box::from(i, j, &minefield);
    let checkable_boxes = mine_box.generate_checkable_boxes(i, j);

    let count: u32 = checkable_boxes
        .iter()
        .map(|f| if minefield[f[0]][f[1]] == '*' { 1 } else { 0 })
        .sum();

    if count == 0 {
        " ".to_string()
    } else {
        count.to_string()
    }
}
