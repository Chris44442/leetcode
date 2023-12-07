fn part1_stuff() {
    const INPUT : &str = include_str!("../input");
    let split_str : Vec<&str> = INPUT.split('\n').collect();
    let mut ultrasum : u32 = 0;
    for k in 0..split_str.len()-1 {
        let (_, right_part) = split_str[k].split_at(9);
        let (links, rechts) = right_part.split_at(32);
        let mut left_numbers : Vec<&str> = links.trim().split_whitespace().collect();
        left_numbers.pop();
        let right_numbers : Vec<&str> = rechts.trim().split_whitespace().collect();
        let parsed_left_numbers: Vec<u32> = left_numbers
            .iter()
            .map(|&s| s.parse::<u32>().unwrap())
            .collect();
        let parsed_right_numbers: Vec<u32> = right_numbers
            .iter()
            .map(|&s| s.parse::<u32>().unwrap())
            .collect();
        let mut sum : u32 = 0;
        for i in 0..parsed_right_numbers.len() {
            for j in 0..parsed_left_numbers.len() {
                if parsed_right_numbers[i] == parsed_left_numbers[j] {
                    if sum == 0 {
                        sum = 1;
                    }
                    else {
                        sum *= 2;
                    }
                }
            }
        }
        ultrasum += sum;
    }
    println!("part1 sum: {}", ultrasum);
}

fn part2_stuff() {
    const INPUT : &str = include_str!("../input");
    let split_str : Vec<&str> = INPUT.split('\n').collect();
    let mut u32_array: Vec<u32> = vec![1; split_str.len()-1];
    for k in 0..split_str.len()-1 {
        let (_, right_part) = split_str[k].split_at(9);
        let (links, rechts) = right_part.split_at(32);
        let mut left_numbers : Vec<&str> = links.trim().split_whitespace().collect();
        left_numbers.pop();
        let right_numbers : Vec<&str> = rechts.trim().split_whitespace().collect();
        let parsed_left_numbers: Vec<u32> = left_numbers
            .iter()
            .map(|&s| s.parse::<u32>().unwrap())
            .collect();
        let parsed_right_numbers: Vec<u32> = right_numbers
            .iter()
            .map(|&s| s.parse::<u32>().unwrap())
            .collect();
        let mut sum : u32 = 0;
        for i in 0..parsed_right_numbers.len() {
            for j in 0..parsed_left_numbers.len() {
                if parsed_right_numbers[i] == parsed_left_numbers[j] {
                    sum += 1;
                }
            }
        }
        for _m in 0..sum {
            if k as u32 + sum <= split_str.len() as u32 {
                for _n in 0..u32_array[k] {
                    u32_array[k+sum as usize] += 1;
                }
            }
            sum -= 1;
        }
    }
    let result : u32 = u32_array.iter().sum();
    println!("part2 sum: {}", result);
}
fn main() {
    part1_stuff();
    part2_stuff();
}

