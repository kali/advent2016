#[macro_use]
extern crate scan_fmt;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let screen = run(&input, 50, 6);
    let p1 = screen
        .iter()
        .map(|r| r.iter().filter(|x| **x).count())
        .sum::<usize>();
    dbg!(p1);
    for row in screen.iter() {
        for p in row {
            print!("{}", if *p { "#" } else { " " });
        }
        println!();
    }
}

fn run(input: &str, width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut screen = vec![vec!(false; width); height];
    for line in input.lines() {
        if let Ok((cols, rows)) = scan_fmt!(line, "rect {}x{}", usize, usize) {
            screen[0..rows]
                .iter_mut()
                .for_each(|row| row[0..cols].iter_mut().for_each(|p| *p = true));
        } else if let Ok((row, i)) = scan_fmt!(line, "rotate row y={} by {}", usize, usize) {
            for _ in 0..i {
                let right = screen[row].pop().unwrap();
                screen[row].insert(0, right);
            }
        } else if let Ok((col, i)) = scan_fmt!(line, "rotate column x={} by {}", usize, usize) {
            for _ in 0..i {
                let bot = screen[height - 1][col];
                for r in (0..height - 1).rev() {
                    screen[r + 1][col] = screen[r][col];
                }
                screen[0][col] = bot;
            }
        } else {
            panic!()
        }
    }
    screen
}

#[test]
fn t() {
    dbg!(run("rect 3x2", 7, 3));
    dbg!(run("rect 3x2\nrotate column x=1 by 1", 7, 3));
    dbg!(run("rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4", 7, 3));
    dbg!(run("rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column x=1 by 1", 7, 3));
}
