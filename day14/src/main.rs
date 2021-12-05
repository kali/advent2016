use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input.trim(), false));
    dbg!(run(&input.trim(), true));
}

fn ensure(salt: &str, index: usize, stretching: bool, hashes: &mut Vec<Vec<u8>>) {
    while hashes.len() < index + 1 {
        let mut msg = format!("{}{}", salt, hashes.len());
        let iter = if stretching { 2017 } else { 1 };
        for _ in 0..iter {
            let digest = md5::compute(&msg);
            msg = format!("{:x}", digest);
        }
        hashes.push(msg.into())
    }
}

fn run(salt: &str, stretching: bool) -> usize {
    let mut hashes = vec![];
    let mut keys = vec![];
    for ix in 0.. {
        ensure(salt, ix, stretching, &mut hashes);
        if let Some(l) = hashes[ix]
            .iter()
            .tuple_windows()
            .find(|(a, b, c)| a == b && a == c)
            .map(|(a, _, _)| *a)
        {
            ensure(salt, ix + 1000, stretching, &mut hashes);
            if (1..=1000).any(|i| {
                hashes[ix + i]
                    .iter()
                    .tuple_windows()
                    .any(|(a, b, c, d, e)| *a == l && *b == l && *c == l && *d == l && *e == l)
            }) {
                keys.push(ix);
                if keys.len() == 64 {
                    break;
                }
            }
        }
    }
    keys[63]
}

#[test]
fn t() {
    assert_eq!(run("abc", false), 22728);
}
