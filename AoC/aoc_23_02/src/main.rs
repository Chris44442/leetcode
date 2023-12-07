fn main() {
    const INPUT : &str = include_str!("../input");
    let mut blue : u32;
    let mut red : u32;
    let mut green : u32;
    let mut sum : u32 = 0;
    let mut sum_part2 : u32 = 0;
    let split_str : Vec<&str> = INPUT.split('\n').collect();
    for k in 0..split_str.len()-1 {
        let ssppit : Vec<&str> = split_str[k].split(": ").collect();
        let supersplit : Vec<&str> = ssppit[1].split("; ").collect();
        let game_id : u32 = k as u32 + 1;
        let mut blue_max : u32 = 0;
        let mut red_max : u32 = 0;
        let mut green_max : u32 = 0;
        for j in 0..supersplit.len() {
            let ultrasplit : Vec<&str> = supersplit[j].split(", ").collect();
            for i in 0..ultrasplit.len() {
                let gigasplit : Vec<&str> = ultrasplit[i].split(' ').collect();
                match gigasplit[1] {
                    "blue" => {
                        blue = gigasplit[0].parse::<u32>().unwrap();
                        if blue_max < blue {
                            blue_max = blue;
                        }
                    }
                    "red" => {
                        red = gigasplit[0].parse::<u32>().unwrap();
                        if red_max < red {
                            red_max = red;
                        }
                    }
                    "green" => {
                        green = gigasplit[0].parse::<u32>().unwrap();
                        if green_max < green {
                            green_max = green;
                        }
                    }
                    _ => {}
                }
            }
        }
        if blue_max > 14 || green_max > 13 || red_max > 12 {
        }
        else {
            sum += game_id;
        }
        sum_part2 += blue_max * green_max * red_max;
    }
    println!("Part 1, sum is {}", sum);
    println!("Part 2, sum is {}", sum_part2);
}

