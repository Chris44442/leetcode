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

pub fn part2(input : &str) -> i64{

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

    // let mut go_ahead : bool = true;
    for bla in 0..2 {

        let mut blocks_clone = blocks.clone();
        for i in (0..blocks_clone.len()).rev() {
            if blocks_clone[i] == FREE_SPACE {
                blocks_clone.remove(i);
            }
        }

        // dbg!(&blocks_clone);
        // dbg!(&blocks);
        //
        // dbg!(&blocks_clone.last());

        // TODO big for loop start here
        let mut no_this_id = 0;
        let last_id = *blocks_clone.last().unwrap();
        let mut break_index = 0;
        for i in (0..blocks_clone.len()).rev() {
            if blocks_clone[i] == last_id {
                blocks_clone.remove(i);
                no_this_id += 1;
                break_index = i;
            }
            else {
                break;
            }
        }

        // dbg!(no_this_id);

        let mut k = 0;
        
        let mut start_index = 0;

        for j in 0..blocks.len() {
            if j >= break_index - 1 {
                // TODO?
                // we are done
                // go_ahead = false;

                break;
            }

            if blocks[j] == FREE_SPACE {
                k += 1;
                if k >= no_this_id {
                    start_index = j - k + 1;
                    break;
                }
            } else {
                // free space not big enough
                k = 0;
            }
        }
        for m in start_index..start_index+k {
            blocks[m] = blocks.pop().unwrap();
        }
    }


    dbg!(&blocks);




    let mut acc = 0;
    for i in 0..blocks.len() {
        if blocks[i] != FREE_SPACE {
            acc += i as i64 *blocks[i];
        }
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

    // #[test]
    // fn test_1() {
    //     let input = "2333133121414131402";
    //     assert_eq!(1928, part1(input));
    //
    // }
    #[test]
    fn test_2() {
        let input = "2333133121414131402";
        assert_eq!(part2(input),2858);

    }
}
