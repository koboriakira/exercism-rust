#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: Vec<char>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, &['A', 'C', 'G', 'T']).and_then(|dna| {
            Ok(Dna {
                nucleotides: dna.chars().collect(),
            })
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self
            .nucleotides
            .iter()
            .map(|n| match n {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!()
            })
            .collect();
        Rna::new(&rna).ok().unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate(rna, &['A', 'C', 'G', 'U']).and_then(|rna| {
            Ok(Rna {
                nucleotides: rna.chars().collect(),
            })
        })
    }
}

fn validate<'a>(dna: &'a str, nucleotides: &[char]) -> Result<&'a str, usize> {
    if let Some(position) = dna.chars().position(|c| !nucleotides.contains(&c)) {
        Err(position)
    } else {
        Ok(dna)
    }
}
