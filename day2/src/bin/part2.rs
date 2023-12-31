fn solve(lines: Vec<&str>) -> i64 {
    lines
        .into_iter()
        .map(|l| {
            let mut s = l.split(": ");
            let _ = s.next().unwrap();
            let game = s.next().unwrap();

            let gs = game.split(';').collect::<Vec<_>>();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for g in gs {
                let b = g.split(", ").map(|s| s.trim()).collect::<Vec<_>>();

                for a in b {
                    let r = a.split(' ').collect::<Vec<_>>();

                    let amt = r.first().unwrap();
                    let color = r.get(1).unwrap();

                    let a = amt.parse::<i64>().unwrap();

                    match *color {
                        "blue" => {
                            if blue < a {
                                blue = a;
                            }
                        }
                        "red" => {
                            if red < a {
                                red = a;
                            }
                        }
                        "green" => {
                            if green < a {
                                green = a;
                            }
                        }
                        _ => {}
                    }
                }
            }

            red * blue * green
        })
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../../input.txt");

    let ans = solve(input.lines().collect());

    println!("solved: {:?}", ans)
}
