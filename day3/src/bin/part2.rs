use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Storage {
    value: i64,
    double: bool,
}

fn solve(lines: Vec<&str>) -> i64 {
    let re = Regex::new(r#"(\d+)"#).unwrap();

    let mut map = HashMap::new();

    lines.iter().enumerate().for_each(|(idx, l)| {
        let caps = re.captures_iter(l);
        let ch = lines[idx].chars().collect::<Vec<_>>();

        caps.for_each(|c| {
            c.iter().enumerate().for_each(|(id, cap)| {
                if id % 2 == 0 {
                    return;
                }
                let c = cap.unwrap();

                if c.start() > 0 && is_valid(&ch[c.start() - 1]) {
                    add_or_update(
                        (idx, c.start() - 1),
                        ch[c.range()]
                            .iter()
                            .collect::<String>()
                            .parse::<i64>()
                            .unwrap(),
                        &mut map,
                    );
                    return;
                }
                if c.end() <= (ch.len() - 1) && is_valid(&ch[c.end()]) {
                    add_or_update(
                        (idx, c.end()),
                        ch[c.range()]
                            .iter()
                            .collect::<String>()
                            .parse::<i64>()
                            .unwrap(),
                        &mut map,
                    );
                    return;
                }

                if idx > 0 {
                    let ch_new = lines[idx - 1].chars().collect::<Vec<_>>();

                    if c.start() > 0 && is_valid(&ch_new[c.start() - 1]) {
                        add_or_update(
                            (idx - 1, c.start() - 1),
                            ch[c.range()]
                                .iter()
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap(),
                            &mut map,
                        );
                        return;
                    }

                    if (c.end() + 1) <= (ch.len() - 1) && is_valid(&ch_new[c.end()]) {
                        add_or_update(
                            (idx - 1, c.end()),
                            ch[c.range()]
                                .iter()
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap(),
                            &mut map,
                        );
                        return;
                    }

                    for z in c.range() {
                        if is_valid(&ch_new[z]) {
                            add_or_update(
                                (idx - 1, z),
                                ch[c.range()]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<i64>()
                                    .unwrap(),
                                &mut map,
                            );
                            return;
                        }
                    }
                }
                if idx < lines.len() - 1 {
                    let ch_new = lines[idx + 1].chars().collect::<Vec<_>>();

                    if c.start() > 0 && is_valid(&ch_new[c.start() - 1]) {
                        add_or_update(
                            (idx + 1, c.start() - 1),
                            ch[c.range()]
                                .iter()
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap(),
                            &mut map,
                        );
                        return;
                    }

                    if (c.end() + 1) <= (ch.len() - 1) && is_valid(&ch_new[c.end()]) {
                        add_or_update(
                            (idx + 1, c.end()),
                            ch[c.range()]
                                .iter()
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap(),
                            &mut map,
                        );
                        return;
                    }
                    for z in c.range() {
                        if is_valid(&ch_new[z]) {
                            add_or_update(
                                (idx + 1, z),
                                ch[c.range()]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<i64>()
                                    .unwrap(),
                                &mut map,
                            );
                            return;
                        }
                    }
                };
            })
        })
    });

    map.into_values()
        .map(|v| {
            if v.double {
                return v.value;
            }
            0
        })
        .sum()
}

fn add_or_update(key: (usize, usize), value: i64, map: &mut HashMap<(usize, usize), Storage>) {
    if let Some(v) = map.get(&key) {
        map.insert(
            key,
            Storage {
                value: v.value * value,
                double: true,
            },
        );
        return;
    }

    map.insert(
        key,
        Storage {
            value: value,
            double: false,
        },
    );
}

fn is_valid(c: &char) -> bool {
    *c == '*'
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

        assert_eq!(solve(case.lines().collect()), 467835)
    }
}
