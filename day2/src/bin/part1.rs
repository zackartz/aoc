const RED: i64 = 12;
const GREEN: i64 = 13;
const BLUE: i64 = 14;

fn solve(lines: Vec<&str>) -> i64 {
    lines
        .into_iter()
        .map(|l| {
            let mut s = l.split(": ");
            let id = s.next().unwrap();
            let game = s.next().unwrap();

            let gs = game.split(';').collect::<Vec<_>>();

            let mut valid = true;
            let id = id.replace("Game ", "");

            for g in gs {
                let b = g.split(", ").map(|s| s.trim()).collect::<Vec<_>>();

                for a in b {
                    let r = a.split(' ').collect::<Vec<_>>();

                    let amt = r.first().unwrap();
                    let color = r.get(1).unwrap();

                    if valid {
                        valid = match *color {
                            "blue" => amt.parse::<i64>().unwrap() <= BLUE,
                            "red" => amt.parse::<i64>().unwrap() <= RED,
                            "green" => amt.parse::<i64>().unwrap() <= GREEN,
                            _ => valid,
                        }
                    }
                }
            }

            match valid {
                true => id.parse::<i64>().unwrap(),
                false => 0,
            }
        })
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../../input.txt");

    let ans = solve(input.lines().collect());

    println!("solved: {:?}", ans)
}
