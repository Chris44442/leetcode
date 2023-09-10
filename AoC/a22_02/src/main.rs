use std::fs;
extern crate memmap;
use memmap::MmapOptions;

fn main() {
  let f = fs::OpenOptions::new().read(true).write(true).open("./input").unwrap();
  let mmap = unsafe {MmapOptions::new().map_mut(&f).unwrap()};
  let bytes: &[u8] = mmap.get(0..).unwrap();

  let mut score:u32 = 0;
  for (ii, _byte) in bytes.iter().enumerate() {
    if bytes[ii] == 0x43 { // C, scissor 1
      if bytes[ii+2] == 0x5a { // Z, scissor 2
        score += 6;}
      else if bytes[ii+2] == 0x59 { // Y, paper 2
        score += 2;}
      else if bytes[ii+2] == 0x58 { // X, rock 2
        score += 7;}}
    else if bytes[ii] == 0x41 { // A, rock 1
      if bytes[ii+2] == 0x5a { // Z, scissor 2
        score += 3;}
      else if bytes[ii+2] == 0x58 { // X, rock 2
        score += 4;}
      else if bytes[ii+2] == 0x59 { // Y, paper 2
        score += 8;}}
    else if bytes[ii] == 0x42 { // B, paper 1
      if bytes[ii+2] == 0x59 { // Y, paper 2
        score += 5;}
      else if bytes[ii+2] == 0x58 { // X, rock 2
        score += 1;}
      else if bytes[ii+2] == 0x5a { // Z, scissor 2
        score += 9;}}
  }
  println!("Part 1 score is {}", score);

  let mut score:u32 = 0;
  for (ii, _byte) in bytes.iter().enumerate() {
    if bytes[ii] == 0x43 { // C, scissor 1
      if bytes[ii+2] == 0x5a { // Z, win, rock
        score += 7;}
      else if bytes[ii+2] == 0x59 { // Y, draw, scissor
        score += 6;}
      else if bytes[ii+2] == 0x58 { // X, lose, paper
        score += 2;}}
    else if bytes[ii] == 0x41 { // A, rock 1
      if bytes[ii+2] == 0x5a { // Z, win, paper
        score += 8;}
      else if bytes[ii+2] == 0x58 { // X, lose, scissor
        score += 3;}
      else if bytes[ii+2] == 0x59 { // Y, draw, rock
        score += 4;}}
    else if bytes[ii] == 0x42 { // B, paper 1
      if bytes[ii+2] == 0x59 { // Y, draw, paper 2
        score += 5;}
      else if bytes[ii+2] == 0x58 { // X, rock 2
        score += 1;}
      else if bytes[ii+2] == 0x5a { // Z, scissor 2
        score += 9;}}
  }
  println!("Part 2 score is {}", score);

}