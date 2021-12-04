fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn run(input: &str) -> (String, String) {
    let mut pwd1 = String::new();
    let mut pwd2 = vec![b'_'; 8];
    let mut context = md5::Context::new();
    context.consume(input);
    for ix in 0.. {
        let mut context = context.clone();
        context.consume(ix.to_string());
        let hash = context.compute();
        let hex = format!("{:x}", hash);
        let hex = hex.as_bytes();
        if hex.iter().take(5).all(|c| *c == b'0') {
            if pwd1.len() < 8 {
                pwd1.push(hex[5] as char);
            }
            let pos = hex[5];
            if pos >= b'0' && pos <= b'7' {
                let pos = pos - b'0';
                if pwd2[pos as usize] == b'_' {
                    pwd2[pos as usize] = hex[6];
                }
            }
            if !pwd2.contains(&b'_') {
                break;
            }
        }
    }
    (pwd1, String::from_utf8(pwd2).unwrap())
}

#[test]
fn t() {
    assert_eq!(run("abc"), ("18f47a30".into(), "05ace8e3".into()));
}
