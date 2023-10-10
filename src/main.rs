fn main() {}
#[allow(dead_code)]

fn inv_sqrt (x: f64) -> f64 { // Quake III "Fast inverse square root"
  const MAGIC_U64: u64 = 0x5fe6ec85e7de30da;
  let x2: f64 = x * 0.5;
  let u1: u64 = unsafe {core::intrinsics::transmute(x)};
  let u2: u64 = MAGIC_U64 - (u1 >> 1);
  let mut inv_sqrt: f64 = unsafe {core::intrinsics::transmute(u2)};
  inv_sqrt *= 1.5 - (x2 * inv_sqrt * inv_sqrt);
  inv_sqrt *= 1.5 - (x2 * inv_sqrt * inv_sqrt);
  inv_sqrt
}

#[cfg(test)]
mod tests {
use super::*;
  #[test]
  fn inv_sqrt_tb() {
    let x = 5613.096;
    assert!((inv_sqrt(x) - 1.0/x.sqrt()).abs() < 0.0000001);
  }
}