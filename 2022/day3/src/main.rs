use std::collections::HashSet;

fn main() {
    let file_content = include_str!("../problem.txt").trim();
    let rucksacks: Vec<_> = file_content.split('\n').collect();

    let mut score = 0;
    for rucksack in &rucksacks {
        let rucksack_items_len = rucksack.chars().count() / 2;
        let first_compartment: HashSet<_> = rucksack[0..rucksack_items_len].chars().collect();
        let second_compartment: HashSet<_> = rucksack[rucksack_items_len..].chars().collect();

        let &misplaced_item = first_compartment
            .intersection(&second_compartment)
            .next()
            .unwrap();

        if misplaced_item.is_ascii_lowercase() {
            score += misplaced_item as i32 - 'a' as i32 + 1;
        } else {
            score += misplaced_item as i32 - 'A' as i32 + 27;
        }
    }

    println!("score: {}", score);

    score = 0;
    for rucksack_group in &mut rucksacks.chunks(3) {
        let mut sets = rucksack_group
            .iter()
            .map(|rucksack| rucksack.chars().collect::<HashSet<_>>());

        let (first_set, second_set, third_set) = (
            sets.next().unwrap(),
            sets.next().unwrap(),
            sets.next().unwrap(),
        );
        let t = first_set.intersection(&second_set);
        let &badge = t.into_iter().find(|c| third_set.contains(c)).unwrap();

        if badge.is_ascii_lowercase() {
            score += badge as i32 - 'a' as i32 + 1;
        } else {
            score += badge as i32 - 'A' as i32 + 27;
        }
    }

    println!("score: {}", score);
}
