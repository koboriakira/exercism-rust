use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'T', 'C', 'G'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    if let Some(illegal_nucleotide) = find_illegal_nucleotide(dna) {
        return Err(illegal_nucleotide);
    }

    let count = dna
        .chars()
        .filter(|d| nucleotide.eq(d))
        .collect::<Vec<char>>()
        .len();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(illegal_nucleotide) = find_illegal_nucleotide(dna) {
        return Err(illegal_nucleotide);
    }
    let mut result: HashMap<char, usize> = HashMap::new();
    NUCLEOTIDES.iter().for_each(|&nucleotide| {
        let count = count(nucleotide, dna);
        result.insert(nucleotide, count.expect(""));
    });
    Ok(result)
}

fn find_illegal_nucleotide(dna: &str) -> Option<char> {
    dna.chars().find(|d| !NUCLEOTIDES.contains(d))
}
