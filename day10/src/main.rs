use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone)]
enum Dst {
    Bot(usize),
    Output(usize),
}
use Dst::*;

#[derive(Copy, Clone)]
struct BotRule {
    low: Dst,
    high: Dst,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut bots = HashMap::<usize, BotRule>::new();
    let mut inputs = HashMap::<usize, usize>::new();
    for line in input.lines() {
        if let Ok((bot, low_type, low_dst, high_type, high_dst)) = scan_fmt!(
            line,
            "bot {} gives low to {} {} and high to {} {}",
            usize,
            String,
            usize,
            String,
            usize
        ) {
            let low = if low_type == "bot" {
                Bot(low_dst)
            } else {
                Output(low_dst)
            };
            let high = if high_type == "bot" {
                Bot(high_dst)
            } else {
                Output(high_dst)
            };
            bots.insert(bot, BotRule { low, high });
        } else if let Ok((value, bot)) = scan_fmt!(line, "value {} goes to bot {}", usize, usize) {
            inputs.insert(value, bot);
        }
    }
    let mut bots_chips = HashMap::<usize, HashSet<usize>>::new();
    let mut outputs_chips = HashMap::<usize, usize>::new();
    for (value, bot) in inputs {
        bots_chips.entry(bot).or_default().insert(value);
    }
    loop {
        let before: usize =
            outputs_chips.len() + bots_chips.values().map(|h| h.len()).sum::<usize>();
        for (&bot, &BotRule { low, high }) in &bots {
            if bots_chips.entry(bot).or_default().len() == 2 {
                let vlow = *bots_chips[&bot].iter().min().unwrap();
                let vhigh = *bots_chips[&bot].iter().max().unwrap();
                for (dst, value) in [(low, vlow), (high, vhigh)] {
                    match dst {
                        Bot(b) => {
                            bots_chips.entry(b).or_default().insert(value);
                        }
                        Output(o) => {
                            outputs_chips.insert(o, value);
                        }
                    };
                }
            }
        }
        let after: usize =
            outputs_chips.len() + bots_chips.values().map(|h| h.len()).sum::<usize>();
        if after == before {
            break;
        }
    }
    let p1 = bots_chips
        .iter()
        .find(|(_, chips)| chips.contains(&17) & chips.contains(&61))
        .unwrap()
        .0;
    dbg!(p1);
    let p2 = outputs_chips[&0] * outputs_chips[&1] * outputs_chips[&2];
    dbg!(p2);
}
