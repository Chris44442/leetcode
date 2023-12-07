fn expand_stuff() -> Vec<String> {
    const INPUT : &str = include_str!("../input_o");
    let split_str : Vec<&str> = INPUT.split('\n').collect();
    let number = split_str[0].len();
    let bla = ".".repeat(number+2);
    let mut exp_split_str : Vec<String> = Vec::new();
    exp_split_str.push(bla.clone());
    for i in 0..split_str.len()-1 {
        let hehe : String = format!(".{}.", split_str[i]);
        exp_split_str.push(hehe);
    }
    exp_split_str.push(bla);
    exp_split_str
}

fn check_vicinity(exp_s:&Vec<String>, index:usize, i:usize) -> bool {
    let mut result : bool = false;
    for k in i-1..=i+1 {
        let bla = exp_s[k].chars().skip(index-1).take(3);
        for c in bla {
            if c.is_digit(10) || c == '.' {
            }
            else {
                result = true;
            }
        }
    }
    result
}

fn part1(exp_s: Vec<String>) -> u32 {
    let mut vicinity_has_symbol : bool = false;
    let mut ultrasum : u32 = 0;
    let mut last_symbol_was_digit : bool = false;
    let mut sum : u32 = 0;
    for i in 0..exp_s.len() {
        for (index, c) in exp_s[i].chars().enumerate() {
            if c.is_digit(10) {
                sum = sum*10 + c.to_digit(10).unwrap();
                if last_symbol_was_digit {
                    vicinity_has_symbol = check_vicinity(&exp_s, index, i) || vicinity_has_symbol;
                }
                else {
                    vicinity_has_symbol = check_vicinity(&exp_s, index, i);
                }
                last_symbol_was_digit = true;
            }
            else {
                if last_symbol_was_digit {
                }
                if vicinity_has_symbol {
                    ultrasum += sum;
                }
                sum = 0;
                vicinity_has_symbol = false;
                last_symbol_was_digit = false;
            }
        }
    }
    ultrasum
}

fn check_vicinity_for_stars(exp_s:&Vec<String>, index:usize, i:usize) -> (u32, u32) {
    let mut result : (u32,u32) = (0,0);
    for k in i-1..=i+1 {
        let bla = exp_s[k].chars().skip(index-1).take(3);
        for (idx,c) in bla.enumerate() {
            if c == '*' {
                result = (k as u32, index as u32 +idx as u32);
            }
        }
    }
    result
}

fn part2(exp_s: Vec<String>) -> u32 {
    let mut vicinity_has_star : bool = false;
    let mut ultrasum : u32 = 0;
    let mut sum : u32 = 0;
    let mut single_star_coord : (u32,u32);
    let mut star_coord : (u32,u32) = (0,0);
    let mut all_coords : Vec<(u32,u32)> = Vec::new();
    let mut sums : Vec<u32> = Vec::new();
    let mut second_star_found : bool;
    let mut special_index : usize = 0;
    for i in 0..exp_s.len() {
        for (index, c) in exp_s[i].chars().enumerate() {
            if c.is_digit(10) {
                sum = sum*10 + c.to_digit(10).unwrap();
                single_star_coord = check_vicinity_for_stars(&exp_s, index, i);
                if single_star_coord != (0,0) {
                    vicinity_has_star = true;
                    star_coord = single_star_coord;
                }
            }
            else {
                if vicinity_has_star {
                    second_star_found = false;
                    for (ind,i) in all_coords.iter().enumerate() {
                        if *i == star_coord {
                            second_star_found = true;
                            special_index = ind;
                        }
                    }
                    if !second_star_found {
                        all_coords.push(star_coord);
                        sums.push(sum);
                    }
                    else {
                        ultrasum += sum * sums[special_index];
                    }
                }
                sum = 0;
                vicinity_has_star = false;
                star_coord = (0,0);
            }
        }
    }
    ultrasum
}

fn main() {
    let exp_s = expand_stuff();
    let ultrasum = part1(exp_s);
    println!("part1 sum: {}", ultrasum);
    let exp_s = expand_stuff();
    let ultrasum2 = part2(exp_s);
    println!("part2 sum: {}", ultrasum2);
}

