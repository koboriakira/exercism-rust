#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { rank, file }),
            (_, _) => None,
        }
    }

    pub fn diff(&self, position: &ChessPosition) -> (i32, i32) {
        (self.rank - position.rank, self.file - position.file)
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let diff = self.position.diff(&(other.position));
        diff.0 == 0 || diff.1 == 0 || diff.0.abs() == diff.1.abs()
    }
}
