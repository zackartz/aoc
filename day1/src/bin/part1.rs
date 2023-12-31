fn solve(lines: Vec<&str>) -> i64 {
    lines
        .iter()
        .map(|l| {
            let c = l
                .chars()
                .filter(|c| c.to_string().parse::<i64>().is_ok())
                .collect::<Vec<_>>();

            format!("{}{}", c.first().unwrap(), c.last().unwrap())
                .parse::<i64>()
                .unwrap()
        })
        .sum::<i64>()
}

fn main() {
    let str = include_str!("../../input/day1.txt");

    println!("solved: {}", solve(str.lines().collect::<Vec<_>>()))
}
