advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list_a, mut list_b) = (Vec::new(), Vec::new());
    input
        .lines()
        .map(|line| line.split_whitespace())
        .for_each(|mut pair| {
            list_a.push(pair.next().unwrap().parse::<i32>().unwrap());
            list_b.push(pair.next().unwrap().parse::<i32>().unwrap());
        });

    list_a.sort();
    list_b.sort();

    let result = list_a
        .into_iter()
        .zip(list_b.into_iter())
        .fold(0, |acc, (left, right)| acc + (left.abs_diff(right)));
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut list_a, mut list_b) = (Vec::new(), Vec::new());
    input
        .lines()
        .map(|line| line.split_whitespace())
        .for_each(|mut pair| {
            list_a.push(pair.next().unwrap().parse::<u32>().unwrap());
            list_b.push(pair.next().unwrap().parse::<u32>().unwrap());
        });

    list_a.sort_unstable();
    list_a.dedup();
    list_b.sort_unstable();

    let mut acc = 0;
    for number in list_a.into_iter() {
        acc += list_b.iter().filter(|n| **n == number).count() as u32 * number;
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
