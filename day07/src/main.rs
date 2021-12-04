use itertools::Itertools;

fn has_abba(s: &[u8]) -> bool {
    s.iter().copied().tuple_windows().any(|(a, b, c, d)| {
        (a as char).is_alphabetic() && (b as char).is_alphabetic() && a != b && a == d && b == c
    })
}

fn abas(s: &[u8]) -> Vec<(u8, u8)> {
    s.iter()
        .copied()
        .tuple_windows()
        .filter(|(a, b, c)| {
            (*a as char).is_alphabetic() && (*b as char).is_alphabetic() && a != b && a == c
        })
        .map(|(a, b, _)| (a, b))
        .collect()
}

fn main() {
    let addrs: Vec<Vec<u8>> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect();
    let splitted: Vec<(Vec<u8>, Vec<u8>)> = addrs
        .iter()
        .map(|addr| {
            let mut inside = false;
            let mut inner = vec![];
            let mut outer = vec![];
            for b in addr.iter().copied() {
                match b {
                    b'[' => {
                        inside = true;
                        outer.push(b'#');
                        inner.push(b'#');
                    }
                    b']' => inside = false,
                    c if inside => inner.push(c),
                    c => outer.push(c),
                }
            }
            (outer, inner)
        })
        .collect();
    let p1 = splitted
        .iter()
        .filter(|(outer, inner)| has_abba(&outer) && !has_abba(&inner))
        .count();
    dbg!(p1);
    let p2 = splitted
        .iter()
        .filter(|(outer, inner)| {
            let o = abas(outer);
            let i = abas(inner);
            o.iter().any(|o| i.iter().any(|i| o.1 == i.0 && o.0 == i.1))
        })
        .count();
    dbg!(p2);
}
