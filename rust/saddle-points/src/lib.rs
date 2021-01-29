use std::collections::HashSet;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let (height, width): (usize, usize) = (input.len(), input[0].len());
    if width == 0 || height == 0 {
        return vec![];
    }

    let pos_has_max_in_rows = (0..height)
        .flat_map(|h| {
            let max = input[h].iter().max().unwrap();
            input[h]
                .iter()
                .enumerate()
                .filter(|(_, v)| v.eq(&max))
                .map(|(i, _)| (h, i))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<HashSet<(usize, usize)>>();
    (0..width)
        .flat_map(|w| {
            let min = (0..height).map(|h| input[h][w]).min().unwrap();
            (0..height)
                .filter(|h| input[*h][w].eq(&min))
                .map(|h| (h, w))
                .collect::<Vec<(usize, usize)>>()
        })
        .filter(|(h, w)| pos_has_max_in_rows.contains(&(*h, *w)))
        .collect::<Vec<(usize, usize)>>()
}
