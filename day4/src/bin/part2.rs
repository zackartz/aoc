use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Card {
    quantity: usize,
    wins: usize,
}

fn solve(lines: Vec<&str>) -> usize {
    let mut copies: BTreeMap<usize, Card> = BTreeMap::new();

    lines.iter().for_each(|l| {
        let mut s = l.split(": ");
        let id = s
            .next()
            .unwrap()
            .replace("Card ", "")
            .trim()
            .parse::<i64>()
            .unwrap();

        let mut nums = s.next().unwrap().split(" | ");
        let winning = nums
            .next()
            .unwrap()
            .split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let wins = nums
            .next()
            .unwrap()
            .split(" ")
            .filter(|x| *x != "")
            .map(|v| {
                if winning.contains(&v.parse::<i64>().unwrap()) {
                    return 1;
                }
                0
            })
            .filter(|x| *x == 1)
            .sum::<usize>();

        copies.insert(id as usize, Card { quantity: 1, wins });
    });

    let mut final_copies = copies.clone();

    let keys = final_copies.keys().map(|k| *k).collect::<Vec<_>>();
    let k = keys.clone();

    for c in k {
        let current_card = final_copies.get(&c).unwrap();
        let current_card = current_card.clone();
        if current_card.wins == 0 {
            continue;
        }

        for z in 0..current_card.wins {
            if let Some(card) = final_copies.get(&(z + c + 1)) {
                final_copies.insert(
                    z + c + 1,
                    Card {
                        quantity: card.quantity + current_card.quantity,
                        wins: card.wins,
                    },
                );
            }
        }
    }

    let mut sorted = final_copies.iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

    final_copies
        .into_values()
        .map(|c| c.quantity)
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../../input.txt");

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

        assert_eq!(solve(case.lines().collect::<Vec<_>>()), 30)
    }
}
