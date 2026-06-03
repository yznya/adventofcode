fn main() {
    let file = include_str!("../problem.txt");

    let mut calories_per_elf: Vec<_> = file
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|x| x.parse::<i32>().unwrap_or_default())
                .sum()
        })
        .collect();

    println!("max: {}", calories_per_elf.iter().max().unwrap());

    calories_per_elf.sort();
    let last_3 = &calories_per_elf[(calories_per_elf.len() - 3)..];
    println!(
        "max 3: {:?}, with sum: {}",
        last_3,
        last_3.iter().sum::<i32>()
    );
}
