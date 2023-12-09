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

fn part1_solution(num:Vec<Vec<Vec<u64>>>) -> u64 {
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

fn part2_solution(num:Vec<Vec<Vec<u64>>>) -> u64 {
    let mut result : u64 = 0;
    for i in 0..u64::MAX {
        let mut source : Vec<u64> = vec![0;8];
        source[7] = i;
        for j in (0..7).rev() {
            source[j] = source[j+1];
            for k in 0..num[j+1].len() {
                if source[j+1] >= num[j+1][k][0] && source[j+1] < num[j+1][k][0] + num[j+1][k][2] {
                    source[j] = source[j+1] - num[j+1][k][0] + num[j+1][k][1];
                }
            }
        }
        if source[0] >= num[0][0][0] && source[0] < num[0][0][0]+num[0][0][1] 
        || source[0] >= num[0][0][2] && source[0] < num[0][0][2]+num[0][0][3] 
        || source[0] >= num[0][0][4] && source[0] < num[0][0][4]+num[0][0][5] 
        || source[0] >= num[0][0][6] && source[0] < num[0][0][6]+num[0][0][7] 
        || source[0] >= num[0][0][8] && source[0] < num[0][0][8]+num[0][0][9] 
        || source[0] >= num[0][0][10] && source[0] < num[0][0][10]+num[0][0][11] 
        || source[0] >= num[0][0][12] && source[0] < num[0][0][12]+num[0][0][13] 
        || source[0] >= num[0][0][14] && source[0] < num[0][0][14]+num[0][0][15] 
        || source[0] >= num[0][0][16] && source[0] < num[0][0][16]+num[0][0][17] 
        || source[0] >= num[0][0][18] && source[0] < num[0][0][18]+num[0][0][19] 
        {
            result = source[7];
            break;
        }
    }
    result
}

fn main() {
    let num = parse_input();
    println!("part1 minimum: {}", part1_solution(num));

    let num = parse_input();
    println!("part2 minimum: {}", part2_solution(num));

    // println!("part2 minimum: {}", part2_solution());
}

