use scan_fmt::scan_fmt;

#[derive(Copy, Clone)]
enum Rhs {
    Imm(i64),
    Reg(char),
}
use Rhs::*;

#[derive(Copy, Clone)]
enum Op {
    Copy(Rhs, char),
    Inc(char),
    Dec(char),
    Jnz(Rhs, i64),
}
use Op::*;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let prog: Vec<Op> = input
        .trim()
        .lines()
        .map(|line| {
            fn val(s: &str) -> Rhs {
                if s.chars().nth(0).unwrap().is_alphabetic() {
                    Reg(s.chars().nth(0).unwrap())
                } else {
                    Imm(s.parse::<i64>().unwrap())
                }
            }
            if let Ok((v, r)) = scan_fmt!(line, "cpy {} {[a-d]}", String, char) {
                Copy(val(&v), r)
            } else if let Ok(a) = scan_fmt!(line, "inc {[a-d]}", char) {
                Inc(a)
            } else if let Ok(a) = scan_fmt!(line, "dec {[a-d]}", char) {
                Dec(a)
            } else if let Ok((v, step)) = scan_fmt!(line, "jnz {} {d}", String, i64) {
                Jnz(val(&v), step)
            } else {
                panic!()
            }
        })
        .collect();
    for part in [1, 2] {
        let mut state = vec![0; 4];
        macro_rules! lhs {
            ($c:expr) => {
                state[($c as u8 - b'a') as usize]
            };
        }
        macro_rules! rhs {
            ($c:expr) => {
                match $c {
                    Imm(i) => i,
                    Reg(r) => lhs!(r),
                }
            };
        }
        lhs!('c') = part - 1;
        let mut pc = 0;
        while pc < prog.len() {
            match prog[pc] {
                Copy(a, b) => lhs!(b) = rhs!(a),
                Inc(r) => lhs!(r) += 1,
                Dec(r) => lhs!(r) -= 1,
                _ => (),
            };
            pc = if let Jnz(r, step) = prog[pc] {
                if rhs!(r) != 0 {
                    (pc as i64 + step) as usize
                } else {
                    pc + 1
                }
            } else {
                pc + 1
            };
        }
        dbg!(lhs!('a'));
    }
}
