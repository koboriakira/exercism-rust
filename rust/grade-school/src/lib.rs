use std::collections::{BTreeMap, BTreeSet};
// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    students_as_grade: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students_as_grade: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, name: &str) {
        let students = self
            .students_as_grade
            .entry(grade)
            .or_insert(BTreeSet::new());
        students.insert(name.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students_as_grade.keys().map(|g| *g).collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.students_as_grade.get(&grade) {
            Some(students) => students.iter().map(|s| s.clone()).collect(),
            None => vec![],
        }
    }
}
