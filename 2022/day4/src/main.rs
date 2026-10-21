use std::ops::RangeInclusive;

trait InclusiveRangeExt {
    fn from_str(s: &str) -> Self;
}

impl InclusiveRangeExt for RangeInclusive<u32> {
    fn from_str(s: &str) -> Self {
        let mut r = s.split('-');
        let start: u32 = r.next().unwrap().parse().unwrap();
        let end: u32 = r.next().unwrap().parse().unwrap();

        start..=end
    }
}

fn main() {
    let range_pairs = include_str!("../problem.txt")
        .trim()
        .split('\n')
        .map(|line| {
            let mut ranges = line.split(',');
            let first_range: RangeInclusive<u32> =
                InclusiveRangeExt::from_str(ranges.next().unwrap());
            let second_range: RangeInclusive<u32> =
                InclusiveRangeExt::from_str(ranges.next().unwrap());

            (first_range, second_range)
        });

    let overlap_ranges_count = range_pairs
        .clone()
        .filter(|(first_range, second_range)| {
            first_range.start() >= second_range.start() && first_range.end() <= second_range.end()
                || second_range.start() >= first_range.start()
                    && second_range.end() <= first_range.end()
        })
        .count();

    println!("count: {}", overlap_ranges_count);

    let duplicate_ranges_count = range_pairs
        .filter(|(first_range, second_range)| {
            first_range.start() >= second_range.start() && first_range.end() <= second_range.end()
                || second_range.start() >= first_range.start()
                    && second_range.end() <= first_range.end()
                || first_range.end() >= second_range.start()
                    && first_range.end() <= second_range.end()
                || second_range.start() <= first_range.end()
                    && first_range.start() <= second_range.end()
        })
        .count();

    println!("count: {}", duplicate_ranges_count);
}
