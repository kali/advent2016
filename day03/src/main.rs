use itertools::Itertools;

fn tri(a: usize, b: usize, c: usize) -> bool {
    let hypo = a.max(b).max(c);
    a + b + c - hypo > hypo
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines: Vec<(usize, usize, usize)> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let part1 = lines.iter().filter(|(a, b, c)| tri(*a, *b, *c)).count();
    dbg!(part1);
    let part2:usize = lines
        .iter()
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            let (a,b,c) = chunks.collect_tuple().unwrap();
            tri(a.0, b.0, c.0) as usize + tri(a.1, b.1, c.1) as usize + tri(a.2, b.2, c.2) as usize
        })
        .sum();
    dbg!(part2);
}
