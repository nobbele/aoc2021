use itertools::Itertools;

pub fn part1() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day2.txt")?;
    let mut pos = (0, 0);
    for (cmd, num_str) in input
        .lines()
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
    {
        match cmd {
            "forward" => pos.0 += num_str.parse::<i32>()?,
            "down" => pos.1 += num_str.parse::<i32>()?,
            "up" => pos.1 -= num_str.parse::<i32>()?,
            _ => panic!("Invalid"),
        }
    }
    println!("{}", pos.0 * pos.1);
    Ok(())
}

pub fn part2() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day2.txt")?;
    let mut pos = (0, 0);
    let mut aim = 0;
    for (cmd, num_str) in input
        .lines()
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
    {
        match cmd {
            "forward" => {
                let v = num_str.parse::<i32>()?;
                pos.0 += v;
                pos.1 += v * aim;
            }
            "down" => aim += num_str.parse::<i32>()?,
            "up" => aim -= num_str.parse::<i32>()?,
            _ => panic!("Invalid"),
        }
    }
    println!("{}", pos.0 * pos.1);
    Ok(())
}
