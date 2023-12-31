fn solve(input: &str) -> usize {
    let time_line = input.lines().next().unwrap();
    let distance_line = input.lines().nth(1).unwrap();

    let time = time_line
        .replace("Time:", "")
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let distance = distance_line
        .replace("Distance: ", "")
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let mut count = 0;
    let mut ms_held = 0;

    while ms_held < time {
        let distance_traveled = (time - ms_held) * ms_held;

        if distance_traveled > distance {
            count += 1;
        }

        ms_held += 1;
    }

    count
}

fn main() {
    let input = r#"Time:        38     67     76     73
Distance:   234   1027   1157   1236"#;

    println!("solved: {}", solve(input));
}

mod test {
    use crate::solve;

    #[test]
    fn test() {
        let inpt = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(solve(inpt), 71503);
    }
}
