fn part1() {
    const INPUT: &[u8;21548] = include_bytes!("../input");
    let mut first_int_detected : bool = false;
    let mut sum : u32 = 0;
    let mut last_number : u32 = 0;
    for i in 0..21548 {
        if INPUT[i] > 47 && INPUT[i] < 58 {
            last_number = INPUT[i] as u32 - 48;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 10 {
            sum += last_number;
            first_int_detected = false;
        }
    }
    println!("part1 sum: {}",sum);
}


fn part2() {
    const INPUT: &[u8;21548] = include_bytes!("../input");
    let mut first_int_detected : bool = false;
    let mut sum : u32 = 0;
    let mut last_number : u32 = 0;
    for i in 0..21548 {
        if INPUT[i] == 111 && INPUT[i+1] == 110 && INPUT[i+2] == 101 {
            last_number = 1;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 116 && INPUT[i+1] == 119 && INPUT[i+2] == 111 {
            last_number = 2;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 116 && INPUT[i+1] == 104 && INPUT[i+2] == 114 && INPUT[i+3] == 101 && INPUT[i+4] == 101 {
            last_number = 3;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 102 && INPUT[i+1] == 111 && INPUT[i+2] == 117 && INPUT[i+3] == 114 {
            last_number = 4;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 102 && INPUT[i+1] == 105 && INPUT[i+2] == 118 && INPUT[i+3] == 101 {
            last_number = 5;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 115 && INPUT[i+1] == 105 && INPUT[i+2] == 120 {
            last_number = 6;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 115 && INPUT[i+1] == 101 && INPUT[i+2] == 118 && INPUT[i+3] == 101 && INPUT[i+4] == 110 {
            last_number = 7;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 101 && INPUT[i+1] == 105 && INPUT[i+2] == 103 && INPUT[i+3] == 104 && INPUT[i+4] == 116 {
            last_number = 8;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 110 && INPUT[i+1] == 105 && INPUT[i+2] == 110 && INPUT[i+3] == 101 {
            last_number = 9;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] > 47 && INPUT[i] < 58 {
            last_number = INPUT[i] as u32 - 48;
            if !first_int_detected {
                sum += last_number*10;
                first_int_detected = true;
            }
        }
        if INPUT[i] == 10 {
            sum += last_number;
            first_int_detected = false;
        }
    }
    println!("part2 sum: {}",sum);
}

fn main() {
    part1();
    part2();
}
