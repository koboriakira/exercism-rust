use core::panic;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};
#[derive(Debug, Clone)]
struct SeasonResult {
    name: String,
    win: u8,
    draw: u8,
    lose: u8,
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Draw => MatchResult::Draw,
            MatchResult::Loss => MatchResult::Win,
        }
    }
}

impl SeasonResult {
    fn new(name: String) -> Self {
        Self {
            name: name,
            win: 0,
            draw: 0,
            lose: 0,
        }
    }

    fn add_match(&mut self, result: &MatchResult) {
        match result {
            MatchResult::Win => self.win += 1,
            MatchResult::Draw => self.draw += 1,
            MatchResult::Loss => self.lose += 1,
        }
    }

    fn matches_played(&self) -> u8 {
        self.win + self.draw + self.lose
    }

    fn point(&self) -> u8 {
        self.win * 3 + self.draw
    }
}

impl From<&SeasonResult> for String {
    fn from(season_result: &SeasonResult) -> String {
        fn format_team(string: &str) -> String {
            format!("{:30}", string)
        }

        fn format_point<T: Display>(num: &T) -> String {
            format!("{:>2}", num)
        }

        let team = format_team(&season_result.name);
        let mp = format_point(&season_result.matches_played());
        let win = format_point(&season_result.win);
        let draw = format_point(&season_result.draw);
        let lose = format_point(&season_result.lose);
        let point = format_point(&season_result.point());

        [team, mp, win, draw, lose, point].join(" | ")
    }
}

fn collect_result(match_result: &str) -> (String, String, MatchResult) {
    let el: Vec<&str> = match_result.split(";").collect();
    let result = match el[2] {
        "win" => MatchResult::Win,
        "draw" => MatchResult::Draw,
        "loss" => MatchResult::Loss,
        _ => panic!(""),
    };
    (el[0].to_string(), el[1].to_string(), result)
}

pub fn tally(match_results: &str) -> String {
    let header = create_header();
    let result: Vec<String> = analyze(match_results)
        .iter()
        .sorted_by_key(|a| a.0)
        .sorted_by(|a, b| b.1.point().cmp(&a.1.point()))
        .map(|(_, season_result)| String::from(season_result))
        .collect();
    [vec![header], result].concat().join("\n")
}

fn analyze(match_results: &str) -> HashMap<String, SeasonResult> {
    match_results
        .split("\n")
        .fold(HashMap::new(), |mut map, match_result| {
            if match_result.is_empty() {
                return map;
            }
            let (hero, villain, result) = collect_result(match_result);
            map.entry(hero.clone())
                .or_insert(SeasonResult::new(hero))
                .add_match(&result);
            map.entry(villain.clone())
                .or_insert(SeasonResult::new(villain))
                .add_match(&result.reverse());
            map
        })
}

fn create_header() -> String {
    fn team(string: &str) -> String {
        format!("{:30}", string)
    }

    fn point<T: Display>(num: &T) -> String {
        format!("{:>2}", num)
    }

    let team = team("Team");
    let mp = point(&"MP".to_string());
    let win = point(&"W".to_string());
    let draw = point(&"D".to_string());
    let lose = point(&"L".to_string());
    let point = point(&"P".to_string());

    [team, mp, win, draw, lose, point].join(" | ")
}
