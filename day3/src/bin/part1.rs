use std::collections::HashSet;

use regex::Regex;

// num regex = (\d+)
//
fn solve(lines: Vec<&str>) -> i64 {
    let re = Regex::new(r#"(\d+)"#).unwrap();

    let x = lines.iter().enumerate().map(|(idx, l)| {
        let caps = re.captures_iter(l);
        let ch = lines[idx].chars().collect::<Vec<_>>();

        caps.map(|c| {
            c.iter()
                .enumerate()
                .map(|(id, cap)| {
                    if id % 2 == 0 {
                        return 0;
                    }
                    let c = cap.unwrap();

                    if c.start() > 0 && is_valid(&ch[c.start() - 1]) {
                        return ch[c.range()]
                            .iter()
                            .collect::<String>()
                            .parse::<i64>()
                            .unwrap();
                    }
                    if c.end() <= (ch.len() - 1) && is_valid(&ch[c.end()]) {
                        return ch[c.range()]
                            .iter()
                            .collect::<String>()
                            .parse::<i64>()
                            .unwrap();
                    }

                    if idx > 0 {
                        let ch_new = lines[idx - 1].chars().collect::<Vec<_>>();

                        if c.start() > 0 && is_valid(&ch_new[c.start() - 1]) {
                            return ch[c.range()]
                                .iter()
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap();
                        }

                        if (c.end() + 1) <= (ch.len() - 1) && is_valid(&ch_new[c.end()]) {
                            return ch[c.range()]
                                .iter()
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap();
                        }

                        for z in c.range() {
                            if c.start() > 0 && is_valid(&ch_new[c.start() - 1]) {
                                return ch[c.range()]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<i64>()
                                    .unwrap();
                            }
                            if (c.end() + 1) <= (ch.len() - 1) && is_valid(&ch_new[c.end()]) {
                                return ch[c.range()]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<i64>()
                                    .unwrap();
                            }

                            if is_valid(&ch_new[z]) {
                                return ch[c.range()]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<i64>()
                                    .unwrap();
                            }
                        }
                    }
                    if idx < lines.len() - 1 {
                        let ch_new = lines[idx + 1].chars().collect::<Vec<_>>();

                        if c.start() > 0 && is_valid(&ch_new[c.start() - 1]) {
                            return ch[c.range()]
                                .iter()
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap();
                        }

                        if (c.end() + 1) <= (ch.len() - 1) && is_valid(&ch_new[c.end()]) {
                            return ch[c.range()]
                                .iter()
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap();
                        }
                        for z in c.range() {
                            if is_valid(&ch_new[z]) {
                                return ch[c.range()]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<i64>()
                                    .unwrap();
                            }
                        }
                    };
                    0
                })
                .sum::<i64>()
        })
        .sum::<i64>()
    });

    x.sum()
}

fn is_valid(c: &char) -> bool {
    *c != '.' && !c.is_ascii_digit()
}

fn main() {
    let input = include_str!("../../input.txt");

    println!("{:?}", solve(input.lines().collect::<Vec<_>>()))
}

mod test {
    use crate::solve;

    #[test]
    fn test_solve() {
        let case = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(solve(case.lines().collect()), 4361)
    }
}
