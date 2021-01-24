use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};
#[derive(Debug, Clone, Copy)]
struct SeasonResult {
    win: u8,
    draw: u8,
    lose: u8,
}

impl SeasonResult {
    fn new() -> Self {
        Self {
            win: 0,
            draw: 0,
            lose: 0,
        }
    }

    // fn win(mut self) -> Self {
    //     self.win += 1;
    //     self
    // }

    // fn lose(mut self) -> Self {
    //     self.lose += 1;
    //     self
    // }

    // fn draw(mut self) -> Self {
    //     self.draw += 1;
    //     self
    // }

    fn win(&mut self) {
        self.win += 1;
    }

    fn lose(&mut self) {
        self.lose += 1;
    }

    fn draw(&mut self) {
        self.draw += 1;
    }

    fn matches_played(&self) -> u8 {
        self.win + self.draw + self.lose
    }

    fn point(&self) -> u8 {
        self.win * 3 + self.draw
    }
}

fn collect_result(match_result: &str) -> (String, String, &str) {
    let el: Vec<&str> = match_result.split(";").collect();
    match el[2].eq("loss") {
        true => return (el[1].to_string(), el[0].to_string(), el[2]),
        false => return (el[0].to_string(), el[1].to_string(), el[2]),
    }
}

pub fn tally(match_results: &str) -> String {
    let mut result_as_team: HashMap<String, SeasonResult> = HashMap::new();
    match match_results.is_empty() {
        true => {}
        false => {
            match_results.split("\n").for_each(|m| {
                let (winner, loser, result) = collect_result(m);
                match result.eq("draw") {
                    true => {
                        result_as_team
                            .entry(winner)
                            .or_insert(SeasonResult::new())
                            .draw();
                        result_as_team
                            .entry(loser)
                            .or_insert(SeasonResult::new())
                            .draw();
                    }
                    false => {
                        result_as_team
                            .entry(winner)
                            .or_insert(SeasonResult::new())
                            .win();
                        result_as_team
                            .entry(loser)
                            .or_insert(SeasonResult::new())
                            .lose();
                    }
                }
            });
        }
    }
    println!("{:?}", result_as_team);

    let header = create_header();
    let result: Vec<String> = result_as_team
        .iter()
        .sorted_by_key(|a| a.0)
        .sorted_by(|a, b| b.1.point().cmp(&a.1.point()))
        .map(|(team, season_result)| create_line(team, season_result))
        .collect();
    [vec![header], result].concat().join("\n")
}

fn create_header() -> String {
    let team = team("Team");
    let mp = point(&"MP".to_string());
    let win = point(&"W".to_string());
    let draw = point(&"D".to_string());
    let lose = point(&"L".to_string());
    let point = point(&"P".to_string());

    [team, mp, win, draw, lose, point].join(" | ")
}

fn create_line(_team: &str, season_result: &SeasonResult) -> String {
    let team = team(_team);
    let mp = point(&season_result.matches_played());
    let win = point(&season_result.win);
    let draw = point(&season_result.draw);
    let lose = point(&season_result.lose);
    let point = point(&season_result.point());

    [team, mp, win, draw, lose, point].join(" | ")
}

fn team(string: &str) -> String {
    format!("{:30}", string)
}

fn point<T: Display>(num: &T) -> String {
    format!("{:>2}", num)
}
