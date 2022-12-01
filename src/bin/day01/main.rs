fn main() {
    let input = std::fs::read_to_string("src/bin/day01/input.txt")
        .expect("Can't open input.txt");

    let elf_totals: Vec<u32> = input.split("\n\n").map(|s|
        // s is the numbers for one elf
        s.split_terminator('\n').map(|n|
            // n is one number for the current elf
            n.parse::<u32>().expect("invalid number")
        ).sum()
    ).collect();

    let result1 = *elf_totals.iter().max().unwrap();
    println!("Part 1: {}", result1);
    assert_eq!(result1, 70374);
}
