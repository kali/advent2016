use itertools::Itertools;

#[derive(Debug)]
struct Room {
    name: String,
    sector: usize,
    crc: String,
}

impl Room {
    fn is_real(&self) -> bool {
        let mut top: Vec<(usize, char)> = self
            .name
            .chars()
            .sorted()
            .filter(|c| *c != '-')
            .group_by(|k| *k)
            .into_iter()
            .map(|(letter, letters)| (letters.into_iter().count(), letter))
            .collect();
        top.sort_by_key(|pair| (-(pair.0 as isize), pair.1));
        top.iter().take(5).map(|pair| pair.1).collect::<String>() == self.crc
    }
}

fn room(input: &str) -> Room {
    let mut split: Vec<&str> = input.split("-").collect();
    let suffix = split.pop().unwrap();
    let name = split.join("-");
    let (sector, crc) = suffix.split_once("[").unwrap();
    let sector = sector.parse().unwrap();
    let crc = crc.trim_end_matches("]").to_string();
    Room { name, sector, crc }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let rooms: Vec<Room> = input.trim().lines().map(room).collect();
    dbg!(rooms
        .iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector)
        .sum::<usize>());
    for room in rooms {
        if room.is_real() {
            let dec: String = room
                .name
                .chars()
                .map(|c| {
                    if c == '-' {
                        c
                    } else {
                        ((((c as u32 - 'a' as u32) + room.sector as u32) % 26) + 'a' as u32) as u8
                            as char
                    }
                })
                .collect();
            println!("{}: {}", dec, room.sector);
        }
    }
}
