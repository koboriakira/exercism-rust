// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Hash, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(mut self) -> Self {
        match self.d {
            Direction::North => self.d = Direction::East,
            Direction::East => self.d = Direction::South,
            Direction::South => self.d = Direction::West,
            Direction::West => self.d = Direction::North,
        };
        self
    }

    pub fn turn_left(mut self) -> Self {
        match self.d {
            Direction::North => self.d = Direction::West,
            Direction::West => self.d = Direction::South,
            Direction::South => self.d = Direction::East,
            Direction::East => self.d = Direction::North,
        };
        self
    }

    pub fn advance(mut self) -> Self {
        match self.d {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        };
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for i in instructions.chars() {
            self = self.instruction(&i);
        }
        self
    }

    pub fn instruction(self, instruction: &char) -> Self {
        match instruction {
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            'A' => self.advance(),
            _ => self,
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
