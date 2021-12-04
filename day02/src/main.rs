fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut pos = 5;
    let mut code = 0;
    for line in input.lines() {
        for step in line.chars() {
            match step {
                'R' if pos % 3 != 0 => pos += 1,
                'L' if pos % 3 != 1 => pos -= 1,
                'U' if pos > 3 => pos -= 3,
                'D' if pos < 7 => pos += 3,
                _ => (),
            }
        }
        code = code * 10 + pos;
    }
    dbg!(code);
    let mut pos = '5';
    let mut code = String::new();
    for line in input.lines() {
        for step in line.chars() {
            pos = match step {
                'R' if !"149CD".contains(pos) => (pos as u8 + 1) as char,
                'L' if !"125AD".contains(pos) => (pos as u8 - 1) as char,
                'U' => match pos {
                    '3' => '1',
                    'D' => 'B',
                    '6'|'7'|'8' => (pos as u8 - 4) as char,
                    'A'|'B'|'C' => (pos as u8 + '6' as u8 - 'A' as u8) as char,
                    _ => pos,
                },
                'D' => match pos {
                    '1' => '3',
                    'B' => 'D',
                    '2'|'3'|'4' => (pos as u8 + 4) as char,
                    '6'|'7'|'8' => (pos as u8 + 'A' as u8 - '6' as u8) as char,
                    _ => pos,
                },
                _ => pos,
            };
        }
        code.push(pos);
    }
    dbg!(code);
}
