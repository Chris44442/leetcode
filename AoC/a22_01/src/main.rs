// use std::fs::File;
// use std::io::{BufRead, BufReader, Result};

// fn main() -> Result<()> {
//   let file: File = File::open("input")?;
//   let reader = BufReader::new(file);

//   let mut numbers = Vec::new();
//   for line in reader.lines() {
//     if let Ok(number_str) = line {
//       if let Ok(number) = number_str.trim().parse::<u32>() {
//         numbers.push(number);}
//       else {
//         numbers.push(0);
//       }
//     }
//   }

//   let mut total_kcal: Vec<u32> = vec![0; numbers.len()];
//   let mut elf: usize = 0;
//   for kcal_of_fooditem in &numbers {
//     if *kcal_of_fooditem == 0 { // this equals the additional newline in the input file which indicates the start of data for the next elf
//       elf += 1;
//       continue;
//     }
//     total_kcal[elf] += kcal_of_fooditem;
//   }

//   let mut kcal_of_1st: u32 = 0;
//   let mut kcal_of_2nd: u32 = 0;
//   let mut kcal_of_3rd: u32 = 0;
//   for kcal_of_elf in &total_kcal {
//     if kcal_of_1st < *kcal_of_elf {
//       kcal_of_3rd = kcal_of_2nd;
//       kcal_of_2nd = kcal_of_1st;
//       kcal_of_1st = *kcal_of_elf;}
//     else if kcal_of_2nd < *kcal_of_elf {
//       kcal_of_3rd = kcal_of_2nd;
//       kcal_of_2nd = *kcal_of_elf;}
//     else if kcal_of_3rd < *kcal_of_elf {
//       kcal_of_3rd = *kcal_of_elf;
//     }
//   }
//   println!("Total kcal of the top 1 elf is: {}",kcal_of_1st);
//   println!("Total kcal of the top 3 elves is: {}",kcal_of_1st + kcal_of_2nd + kcal_of_3rd);

//   Ok(())
// }

use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input");

fn main() {
  let elves = INPUT
    .split("\n\n")
    .map(|ls| ls.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
    .collect::<BTreeSet<_>>();
  println!("Max Elf: {:?}", elves.iter().rev().next().unwrap());
  println!("Max 3 Elves: {}", elves.iter().rev().take(3).sum::<i32>());
}

// for obco in elves {
//   println!("Max Elf: {}", obco);
// }