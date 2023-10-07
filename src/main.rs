fn main() { // Famous Quake III "Fast inverse square root" algorithm implementation
  let input: f64 = 1234567.0;

  const MAGIC_U64: u64 = 0x5fe6ec85e7de30da;
  const THREEHALFS: f64 = 1.5;
  let x2 = input * 0.5;
  let value: u64 = unsafe {core::intrinsics::transmute(input)}; // evil floating point bit level hacking
  let i = MAGIC_U64 - (value >> 1); // what the fuck?
  let mut inv_sqrt: f64 = unsafe {core::intrinsics::transmute(i)};
  inv_sqrt *= THREEHALFS - (x2 * inv_sqrt * inv_sqrt); // Newton–Raphson method
  inv_sqrt *= THREEHALFS - (x2 * inv_sqrt * inv_sqrt); // repeat for more precision

  println!("QIII sqrt {:.12} ", inv_sqrt*input);
  println!("hard sqrt {:.12} ", input.sqrt());
}