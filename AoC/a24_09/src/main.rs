pub fn part1(input : &str) -> i64{

    const FREE_SPACE : i64 = 999_999_999;
    let digits : Vec<i64> = input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut id : i64 = 0;
    let mut is_data : bool = true;
    let mut blocks : Vec<i64> = vec![];
    for digit in digits {
        for _i in 0..digit {
            if is_data {
                blocks.push(id);
            }
            else {
                blocks.push(FREE_SPACE);
            }
        }
        if is_data {
            id += 1;
        }
        is_data =  !is_data;
    }

    let mut blocks_clone = blocks.clone();
    for i in (0..blocks_clone.len()).rev() {
        if blocks_clone[i] == FREE_SPACE {
            blocks_clone.remove(i);
        }
    }

    for j in 0..blocks_clone.len() {
        for i in (0..blocks.len()).rev() {
            if blocks[i] == FREE_SPACE {
                blocks.remove(i);
            }
            else {
                break;
            }
        }
        if blocks[j] == FREE_SPACE {
            blocks[j] = *blocks.last().unwrap();
            blocks.pop();
        }
    }

    let mut acc = 0;
    for i in 0..blocks.len() {
        acc += i as i64 *blocks[i];
    }

    return acc;
}

fn main() {

    let input : &str = include_str!("../input.txt");
    let result = part1(input);
    print!("{:?}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "2333133121414131402";
        assert_eq!(1928, part1(input));

    }
}
