fn extract_numbers(str_of_numbers:&str) -> Vec<u64> {
    str_of_numbers
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn parse_input() -> Vec<Vec<Vec<u64>>> {
    let split_str : Vec<&str> = include_str!("../input")
        .split("\n\n")
        .collect();
    let mut numbers : Vec<Vec<Vec<u64>>> = Vec::new();
    for k in 0..split_str.len() {
        numbers.push(vec![]);
        let supersplit : Vec<&str> = split_str[k].split(":").collect();
        if k == 0 {
            numbers[k].push(extract_numbers(supersplit[1]));
        }
        else {
            let ultrasplit : Vec<&str> = supersplit[1].split("\n").collect();
            for i in 1..ultrasplit.len() {
                numbers[k].push(extract_numbers(ultrasplit[i]));
            }
        }
    }
    numbers[7].pop(); // annoying workaround for last line special case
    numbers
}

fn part1_solution() -> u64 {
    let num = parse_input();
    let mut destination : Vec<Vec<u64>> = vec![vec![0; num.len()]; num[0][0].len()];
    for i in 0..num[0][0].len() {
        destination[i][0] = num[0][0][i];
        for j in 1..num.len() {
            destination[i][j] = destination[i][j-1];
            for k in 0..num[j].len() {
                if destination[i][j-1] >= num[j][k][1] && destination[i][j-1] < num[j][k][1] + num[j][k][2] {
                    destination[i][j] = destination[i][j-1] - num[j][k][1] + num[j][k][0];
                }
            }
        }
    }
    destination.iter().map(|row| row[7]).min().unwrap()
}

fn main() {
    println!("part1 minimum: {}", part1_solution());
}

