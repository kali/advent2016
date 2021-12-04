use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();
    for part in [1, -1] {
        let p: String = (0..lines[0].len())
            .map(|ix| {
                lines
                    .iter()
                    .map(|l| l.as_bytes()[ix])
                    .sorted()
                    .group_by(|k| *k)
                    .into_iter()
                    .map(|(k, ks)| (ks.into_iter().count(), k))
                    .max_by_key(|k| k.0 as i32 * part)
                    .unwrap()
                    .1 as char
            })
        .collect();
        dbg!(p);
    }
}
