fn calc_number_of_wins(t:&f64,d:&f64) -> f64 {
    let x1 : f64 = t/2.0 + ((t*t/4.0)-d).sqrt();
    let x2 : f64 = t/2.0 - ((t*t/4.0)-d).sqrt();
    (x1-f64::MIN_POSITIVE).floor() - (x2-f64::MIN_POSITIVE).floor()
}

fn main() {
    let lines : Vec<&str> = include_str!("../input")
        .split_terminator("\n")
        .collect();
    let time : Vec<f64> = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();
    let product : f64 = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .zip(time.iter())
        .map(|(a, b)| calc_number_of_wins(&b,&a))
        .product();
    println!("Part 1: {}", product);
    println!("Part 2: {}", calc_number_of_wins(&54946592.0, &302147610291404.0));
}

// d = t2*v
// t = t1+t2
// v = t1*1[m/s^2]
// t2 = 7-t1 = 7-v
// v = 7-t2
// d = t2 * (7-t2)
// d = (7-t1) * (t1)
// d = -t1^2 + tgeg * t1
// 0 = -t1^2 + tgeg * t1 -d
// 0 = t1^2 - tgeg * t1 + dgeg
// x1 = -p/2 + sqrt((-p/2)^2-q)
// x1 = tgeg/2 + sqrt((tgeg/2)^2-dgeg)
// Time:      7  15   30
// Distance:  9  40  200

