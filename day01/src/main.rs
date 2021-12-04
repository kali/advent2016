fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut pos = num_complex::Complex::new(0i64, 0);
    let mut dir = num_complex::Complex::new(1, 0);
    let i = num_complex::Complex::new(0, 1);
    for step in input.trim().split(", ") {
        let turn = step.chars().nth(0).unwrap();
        dir = dir * if turn == 'R' { i } else { i.conj() };
        pos += dir * step.trim_start_matches(char::is_alphabetic).parse::<i64>().unwrap();
    }
    dbg!(pos.im + pos.re);
    let mut pos = num_complex::Complex::new(0i64, 0);
    let mut been = std::collections::HashSet::new();
    'top: for step in input.trim().split(", ") {
        let turn = step.chars().nth(0).unwrap();
        dir = dir * if turn == 'R' { i } else { i.conj() };
        for _ in 0..step.trim_start_matches(char::is_alphabetic).parse::<i64>().unwrap() {
            pos += dir;
            if been.contains(&pos) {
                break 'top;
            }
            been.insert(pos);
        }
    }
    dbg!(pos.im + pos.re);
}
