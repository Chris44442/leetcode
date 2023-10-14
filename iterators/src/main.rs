


#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

fn main() {
  // cool iterator over stuff. Nice!
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  for value in v1_iter {
    println!("Got: {}", value);
  }
}

#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter();

// others: iter_mut(), into_iter()
// adapters: take iterator, give another iterator
// consumers: take iterator, give another type, e.g. integer

  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
  let v1 = [1, 2, 3];
  let inteeg: i32 = v1.iter().sum();
  assert_eq!(inteeg, 6);
}

#[test]
fn mapping_stuff() {
  let v2: Vec<i32> = vec![1, 2, 3];
  let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
  assert_eq!(v3, vec![2, 3, 4]);
}

#[test]
fn filters_by_size() {
  let shoes = vec![
    Shoe {
      size: 10,
      style: String::from("sneaker"),
    },
    Shoe {
      size: 13,
      style: String::from("sandal"),
    },
    Shoe {
      size: 10,
      style: String::from("boot"),
    },
  ];

  let in_my_size = shoes_in_my_size(shoes, 10);

  assert_eq!(in_my_size, vec![
    Shoe {
      size: 10,
      style: String::from("sneaker")
    },
    Shoe {
      size: 10,
      style: String::from("boot")
    },
  ])
}

#[test]
  fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
  }

#[test]
  fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
      .zip(Counter::new().skip(1))
      .map(|(a, b)| a * b)
      .filter(|x| x % 3 == 0)
      .sum();
    assert_eq!(18, sum);
  }