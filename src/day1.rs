use itertools::Itertools;

pub fn part1() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day1.txt")?;
    let count = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();
    println!("{}", count);
    Ok(())
}

pub fn part2() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day1txt")?;
    let count = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();
    println!("{}", count);
    Ok(())
}
