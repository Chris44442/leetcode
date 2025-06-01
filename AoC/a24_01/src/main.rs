
pub fn part1(input : &str) -> i64{

    let lines = input.lines();
    let mut left1 = vec![];
    let mut right1 = vec![];
    for line in lines {
        let mut items = line.split_whitespace();
        left1.push(items.next().unwrap().parse::<i64>().unwrap());
        right1.push(items.next().unwrap().parse::<i64>().unwrap());
    }

    left1.sort();
    right1.sort();

    let mut acc = 0;
    for i in 0..left1.len() {
        acc += (left1[i] - right1[i]).abs();
    }

    return acc;
}

fn main() {

    // let lines : Vec<&str> = include_str!("../input")
    //     .split_terminator("\n")
    //     .collect();
    let input : &str = include_str!("../input.txt");
    let result = part1(input);

    print!("{:?}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(11, part1(input));

    }
}
