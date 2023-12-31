use std::{collections::BTreeMap, ops::Range};

use indicatif::ParallelProgressIterator;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

#[derive(Debug, Eq, PartialEq, Hash)]
struct SeedRange {
    from: Range<usize>,
    to: Range<usize>,
}

fn solve(input: &str) -> usize {
    let mut map: BTreeMap<String, Vec<SeedRange>> = BTreeMap::new();

    let mut groups = input.split("\n\n");
    let seeds = groups.next();

    groups.for_each(|l| {
        let mut lines = l.lines();
        let id = lines.next().unwrap().replace(" map:", "");

        if map.get(&id).is_none() {
            map.insert(id.clone(), Vec::new());
        };

        let submap = map.get_mut(&id).unwrap();

        lines.for_each(|value| {
            let values = value
                .split(" ")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let dest = values.get(0).unwrap();
            let src = values.get(1).unwrap();
            let dist = values.get(2).unwrap();

            submap.push(SeedRange {
                from: *src..*src + dist,
                to: *dest..*dest + dist,
            });
        })
    });

    let s = seeds
        .unwrap()
        .replace("seeds: ", "")
        .split(" ")
        .collect::<Vec<_>>()
        .chunks(2)
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map(|c| {
            let mut itr = c.iter();
            let base = itr.next().unwrap().parse::<usize>().unwrap();
            let range = itr.next().unwrap().parse::<usize>().unwrap();

            Range {
                start: base,
                end: base + range,
            }
        })
        .into_par_iter()
        .map(|seed| parse_seed(&map, seed))
        .min();

    s.unwrap()
}

fn parse_seed(map: &BTreeMap<String, Vec<SeedRange>>, seed: usize) -> usize {
    let steps = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut v = seed;

    for s in steps {
        let m = map.get(s);
        let prev = v;

        if let Some(x) = m {
            for sr in x {
                if sr.from.contains(&v) {
                    // println!(
                    //     "---- {} is contained in {}-{}",
                    //     v, sr.from.start, sr.from.end
                    // );
                    let init = sr.from.start;
                    let diff = v - sr.from.start;
                    let val = sr.to.start + diff;

                    v = val;
                    break;
                }
            }
        }

        // println!("{:?}: {:?} -> {:?}", s, prev, v);
    }

    // println!("=======> seed: {:?} -> {:?}", seed, v);

    v
}

fn main() {
    let inp = include_str!("../../input.txt");
    println!("solved: {}", solve(inp));
}

mod test {
    use crate::solve;

    #[test]
    fn test() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        assert_eq!(solve(input), 35);

        // println!("{:?}", solve(input))
    }
}
