#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_frame: usize,
    current_pins: Vec<u16>,
    pins_results: Vec<Vec<u16>>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_frame: 1,
            current_pins: vec![],
            pins_results: vec![],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.current_frame > 10 {
            return Err(Error::GameComplete);
        }
        self.current_pins.push(pins);
        match (
            self.current_frame,
            self.throw_times(),
            self.pins_in_current_frame(),
        ) {
            (1..=9, 1, 0..=9) => {}
            (1..=9, 1, 10) => self.next_frame(),
            (1..=9, 2, 0..=10) => self.next_frame(),
            (1..=9, _, _) => return Err(Error::NotEnoughPinsLeft),
            (10, 1, _) => {}
            (10, 2, 0..=9) => self.next_frame(),
            (10, 2, 10..=20) => match (
                self.current_pins.get(0),
                self.current_pins.iter().sum::<u16>(),
            ) {
                (Some(10), _) => {}
                (_, 10) => {}
                (_, _) => return Err(Error::NotEnoughPinsLeft),
            },
            (10, 3, _) => {
                let first = self.current_pins.get(0).unwrap();
                let second = self.current_pins.get(1).unwrap();
                let third = self.current_pins.get(2).unwrap();
                match (first, first + second, second + third, third) {
                    (10, 20, _, _) => self.next_frame(),
                    (10, 10..=19, 11..=20, _) => return Err(Error::NotEnoughPinsLeft),
                    (0..=9, 10, _, _) => self.next_frame(),
                    (0..=9, 11..=20, _, _) => return Err(Error::NotEnoughPinsLeft),
                    (_, _, _, _) => self.next_frame(),
                }
            }
            (_, _, _) => return Err(Error::NotEnoughPinsLeft),
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        fn calculate_score(three_frames_results: &[Vec<u16>]) -> u16 {
            println!("three_frames_results: {:?}", three_frames_results);
            let first_frame = three_frames_results.get(0).unwrap();
            let flatten_pins: Vec<u16> =
                three_frames_results.iter().flatten().map(|p| *p).collect();
            match (first_frame.len(), first_frame.iter().sum()) {
                (1, 10) => match (flatten_pins.get(1), flatten_pins.get(2)) {
                    (Some(first), Some(second)) => 10 + first + second,
                    (Some(first), None) => 10 + first,
                    (_, _) => 10,
                },
                (2, 10) => match flatten_pins.get(2) {
                    Some(first) => 10 + first,
                    None => 10,
                },
                (_, _) => first_frame.iter().sum(),
            }
        }

        match self.current_frame == 11 {
            true => {
                let scores_until_eighth_frame: u16 = self
                    .pins_results
                    .windows(3)
                    .map(|three_frames_results| calculate_score(three_frames_results))
                    .sum();
                let score_at_ninth_frame = calculate_score(&[
                    self.pins_results.get(8).unwrap().clone(),
                    self.pins_results.get(9).unwrap().clone(),
                ]);
                let score_at_tenth_frame =
                    calculate_score(&[self.pins_results.get(9).unwrap().clone()]);
                Some(scores_until_eighth_frame + score_at_ninth_frame + score_at_tenth_frame)
            }
            _ => None,
        }
    }

    fn next_frame(&mut self) {
        self.pins_results.push(self.current_pins.clone());
        self.current_frame += 1;
        self.current_pins.clear();
    }

    fn throw_times(&self) -> usize {
        self.current_pins.len()
    }

    fn pins_in_current_frame(&self) -> u16 {
        self.current_pins.iter().sum()
    }
}
