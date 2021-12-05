fn moves(input: i64, x: i64, y: i64) -> Vec<(i64, i64)> {
    [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
        .into_iter()
        .filter(|(x, y)| {
            *x >= 0
                && *y >= 0
                && (x * x + 3 * x + 2 * x * y + y + y * y + input as i64).count_ones() % 2 == 0
        })
        .collect()
}

fn path_to(input: i64, x0: i64, y0: i64) -> usize {
    pathfinding::directed::bfs::bfs(
        &(1i64, 1i64),
        |&(x, y)| moves(input, x, y),
        |&(x, y)| x == x0 && y == y0,
    )
    .unwrap()
    .len() as usize
        - 1
}

fn main() {
    let input = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    dbg!(&path_to(input, 31, 39));
    let reach = pathfinding::directed::bfs::bfs_reach((1i64, 1i64), |&(x, y)| {
        moves(input, x, y)
            .into_iter()
            .filter(move |_| x < 100 && y < 100)
            .map(move |(x, y)| (x, y))
    })
    .filter(|(x, y)| path_to(input, *x, *y) <= 50);
    dbg!(reach.count());
}
