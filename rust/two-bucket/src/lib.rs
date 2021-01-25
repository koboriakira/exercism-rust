use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl BucketStats {
    fn new(moves: u8, goal_bucket: Bucket, other_bucket: u8) -> Self {
        BucketStats {
            moves,
            goal_bucket,
            other_bucket,
        }
    }

    fn inverse(self) -> Self {
        BucketStats {
            goal_bucket: match self.goal_bucket {
                Bucket::One => Bucket::Two,
                Bucket::Two => Bucket::One,
            },
            ..self
        }
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    match start_bucket {
        Bucket::One => sub_solve(capacity_1, capacity_2, goal),
        Bucket::Two => match sub_solve(capacity_2, capacity_1, goal) {
            Some(answer) => Some(answer.inverse()),
            _ => None,
        },
    }
}

fn sub_solve(capacity_1: u8, capacity_2: u8, goal: u8) -> Option<BucketStats> {
    match (capacity_1 == goal, capacity_2 == goal) {
        (true, _) => return Some(BucketStats::new(1, Bucket::One, 0)),
        (_, true) => return Some(BucketStats::new(2, Bucket::Two, capacity_1)),
        (_, _) => {}
    }

    let (mut bucket1, mut bucket2, mut moves): (u8, u8, u8) = (0, 0, 0);
    let mut bucket_memory: HashSet<(u8, u8)> = HashSet::new();
    loop {
        moves += 1;
        match (bucket1, bucket2 == capacity_2) {
            (0, _) => bucket1 = capacity_1.clone(),
            (_, true) => bucket2 = 0,
            (_, _) => match bucket1 + bucket2 <= capacity_2 {
                true => {
                    bucket2 = bucket1 + bucket2;
                    bucket1 = 0;
                }
                false => {
                    bucket1 = bucket1 + bucket2 - capacity_2;
                    bucket2 = capacity_2.clone();
                }
            },
        }
        println!("bucket1: {}, bucket2: {}", bucket1, bucket2);

        match (
            bucket_memory.contains(&(bucket1, bucket2)),
            goal == bucket1,
            goal == bucket2,
        ) {
            (true, _, _) => return None,
            (_, true, _) => return Some(BucketStats::new(moves, Bucket::One, bucket2)),
            (_, _, true) => return Some(BucketStats::new(moves, Bucket::Two, bucket1)),
            (_, _, _) => {}
        }

        bucket_memory.insert((bucket1, bucket2));
    }
}
