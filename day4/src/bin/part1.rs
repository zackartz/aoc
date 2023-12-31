fn solve(lines: Vec<&str>) -> i64 {
    lines
        .iter()
        .map(|l| {
            let mut s = l.split(": ");
            let id = s.next().unwrap().replace("Card ", "");
            let mut nums = s.next().unwrap().split(" | ");
            let winning = nums
                .next()
                .unwrap()
                .split(" ")
                .filter(|x| *x != "")
                .collect::<Vec<_>>();

            let mut total = 0;
            nums.next()
                .unwrap()
                .split(" ")
                .filter(|x| *x != "")
                .map(|v| {
                    if winning.contains(&v) {
                        return 1;
                    }
                    0
                })
                .filter(|x| *x == 1)
                .for_each(|_| {
                    if total == 0 {
                        total = 1;
                    } else {
                        total *= 2;
                    }
                });
            total
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    println!("solved: {}", solve(input.lines().collect()));
}

mod test {
    use crate::solve;

    #[test]
    fn test() {
        let case = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(solve(case.lines().collect::<Vec<_>>()), 13)
    }
}
