fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(p1(input.trim()));
    dbg!(p2(input.trim()));
}

fn p1(input: &str) -> usize {
    let mut pos = 0;
    let mut exp = 0;
    let input = input.as_bytes();
    while pos < input.len() {
        let c = input[pos];
        if c == b'(' {
            let prefix_len = input[pos..].iter().position(|c| *c == b')').unwrap();
            let prefix = std::str::from_utf8(&input[pos + 1..][..prefix_len - 1]).unwrap();
            let (len, repeat) = prefix.split_once('x').unwrap();
            let len: usize = len.parse().unwrap();
            let repeat: usize = repeat.parse().unwrap();
            pos += prefix_len + len + 1;
            exp += repeat * len;
        } else {
            pos += 1;
            exp += 1;
        }
    }
    exp
}

fn p2(input: &str) -> usize {
    let mut pos = 0;
    let mut exp = 0;
    let input = input.as_bytes();
    while pos < input.len() {
        let c = input[pos];
        if c == b'(' {
            let prefix_len = input[pos..].iter().position(|c| *c == b')').unwrap();
            let prefix = std::str::from_utf8(&input[pos + 1..][..prefix_len - 1]).unwrap();
            let (len, repeat) = prefix.split_once('x').unwrap();
            let len: usize = len.parse().unwrap();
            let repeat: usize = repeat.parse().unwrap();
            let sub = std::str::from_utf8(&input[pos + prefix_len + 1..][..len]).unwrap();
            exp += repeat * p2(sub);
            pos += prefix_len + 1 + len;
        } else {
            pos += 1;
            exp += 1;
        }
    }
    exp
}

#[test]
fn t1() {
    assert_eq!(p1("X(8x2)(3x3)ABCY"), 18);
}

#[test]
fn t2() {
    assert_eq!(p2("X(8x2)(3x3)ABCY"), 20);
}
