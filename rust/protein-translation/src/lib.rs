use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.map.get(codon) {
            Some(result) => Some(*result),
            _ => None,
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        match rna
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .try_fold(vec![], |mut acc, chunk| {
                match self.name_for(&chunk.into_iter().collect::<String>()[..]) {
                    None => return Err(None),
                    Some(codon) if codon == "stop codon" => Err(Some(acc)),
                    Some(codon) => {
                        acc.push(codon);
                        Ok(acc)
                    }
                }
            }) {
            Ok(result) | Err(Some(result)) => Some(result),
            _ => None,
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        map: pairs.into_iter().collect(),
    }
}
