advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_sum = 0;
    for line in input.lines() {
        let nums: Vec<u32> = line.chars().fold(Vec::<u32>::new(), |mut acc, c| {
            if c.is_digit(10) {
                acc.push(c.to_string().parse::<u32>().unwrap());
            }
            acc
        });
        let num = (10 * nums[0]) + nums[nums.len() - 1];
        total_sum += num;
    }
    //
    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("result: {:?}", result);
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
