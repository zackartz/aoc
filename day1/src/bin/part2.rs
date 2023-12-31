use regex::Regex;

fn parse_number(n: &str) -> i64 {
    let p = n.parse::<i64>();

    match p {
        Ok(n) => n,
        Err(_) => match n {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        },
    }
}

fn solve(lines: Vec<&str>) -> i64 {
    let re =
        Regex::new("(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)").unwrap();

    lines
        .iter()
        .map(|l| {
            let c = re
                .captures_iter(l)
                .map(|cap| parse_number(cap.extract::<1>().0))
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
