fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  for i in 0..nums.len()-1 {
    for j in i+1..=nums.len()-1 {
//        println!("indices: {i} {j}");
      if nums[i]+nums[j] == target {
        return vec![i.try_into().unwrap(), j.try_into().unwrap()];
      }
    }
  }
  return vec![0,0];
}

fn main() {
  let result: Vec<i32>;
  result = two_sum(vec![2,7,11,3],9);
        println!("WINNER: {} {}",result[0],result[1]);
}

