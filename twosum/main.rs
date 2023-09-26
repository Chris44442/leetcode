fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut result: Vec<i32> = Vec::with_capacity(2);
  for i in 0..nums.len()-1 {
    for j in i+1..=nums.len()-1 {
      if nums[i]+nums[j] == target {
        result.push(i as i32);
        result.push(j as i32);
        return result;
      }
    }
  }
  result
}

fn main() {
  let result: Vec<i32>;
  result = two_sum(vec![2,7,11,3],9);
        println!("WINNER: {} {}",result[0],result[1]);
}

