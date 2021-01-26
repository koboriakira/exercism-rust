#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn should_decrement_steps(&self) -> bool {
        [Direction::Right, Direction::Left].contains(self)
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    match size {
        0 => vec![],
        1 => vec![vec![1]],
        _ => {
            let mut result = init(size);
            let mut steps = size.clone() as usize;
            let mut pos = (0_usize, 0_usize);
            let mut direction = Direction::Right;
            let mut count = 1_u32;
            loop {
                println!("steps: {}, pos: {:?}", steps, pos);
                println!("{:?}", result);
                match direction {
                    Direction::Right => {
                        for i in 0..steps {
                            result[pos.0][pos.1 + i] = count;
                            count += 1;
                        }
                        pos = (pos.0 + 1, pos.1 + steps - 1);
                    }
                    Direction::Down => {
                        for i in 0..steps {
                            result[pos.0 + i][pos.1] = count;
                            count += 1;
                        }
                        pos = (pos.0 + steps - 1, pos.1 - 1);
                    }
                    Direction::Left => {
                        for i in 0..steps {
                            result[pos.0][pos.1 - i] = count;
                            count += 1;
                        }
                        pos = (pos.0 - 1, 1 + pos.1 - steps);
                    }
                    Direction::Up => {
                        for i in 0..steps {
                            result[pos.0 - i][pos.1] = count;
                            count += 1;
                        }
                        pos = (1 + pos.0 - steps, pos.1 + 1);
                    }
                };
                steps -= if direction.should_decrement_steps() {
                    1
                } else {
                    0
                };
                if steps == 0 {
                    break;
                }
                direction = direction.turn();
            }
            return result;
        }
    }
}

fn init(size: u32) -> Vec<Vec<u32>> {
    let mut result = vec![];
    for _ in 0..size {
        let mut array = vec![];
        for _ in 0..size {
            array.push(0);
        }
        result.push(array);
    }
    result
}
