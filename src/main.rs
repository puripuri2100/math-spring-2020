use bigint::U128;
use bigint::U256;
use bigint::U512;
use numext_fixed_uint::U2048;

//nPr
fn permutation(n64: u64, r: u64) -> U2048 {
  let n: U2048 = n64.into();
  let mut m: U2048 = U2048::from(1 as u64);
  for i_64 in 0..r {
    let i: U2048 = i_64.into();
    let l: U2048 = m * (n.clone() - i);
    m = l
  }
  m
}

// nCr
// 4C3 = 4*3*2
fn combin(n64: u64, r: u64) -> U2048 {
  let n2: U2048 = permutation(n64, r).into();
  let r2: U2048 = permutation(r, r).into();
  let a = n2 / r2;
  println!("combin {}C{} -> {}", n64, r, a.clone());
  a
}

//n!
fn factorial(n: u64) -> U2048 {
  let mut m: U2048 = U2048::from(1 as u64);
  for i_64 in 1..(n + 1) {
    let i: U2048 = i_64.into();
    m = m * i;
    //println!("m : {}", m)
  }
  println!("factorial : {}! -> {}", n, m);
  m
}

fn number_of_combinations(all: u64, group: u64) -> U2048 {
  // (400C50 * 350C50 * 300C50 * 250C50 * 200C50 * 150C8 * 100C50) / 50!
  let member = all / group;
  let mut n: U2048 = U2048::from(1 as u64);
  for i in 0..group {
    let nokori = all - member * i;
    n = n * combin(nokori, member);
    //println!("n : {}", n);
  }
  println!("main : {}", n);
  let l = factorial(group);
  n / l
}

fn main() {
  println!(
    "Length of combin : {}",
    format!("{}", combin(350, 49)).len()
  );
  println!(
    "Number of number_of_combinations : {}",
    number_of_combinations(350, 7)
  );
  println!(
    "Length of number_of_combinations : {}",
    format!("{}", number_of_combinations(350, 7)).len()
  );
  println!(
    "Number of number_of_combinations : {}",
    number_of_combinations(400, 8)
  );
  println!(
    "Length of number_of_combinations : {}",
    format!("{}", number_of_combinations(400, 8)).len()
  );
}

#[test]

fn check() {
  assert_eq!(combin(4, 2), U2048::from(6 as u64));
  assert_eq!(combin(6, 2), U2048::from(15 as u64));
  assert_eq!(combin(9, 3), U2048::from(280 as u64));

  assert_eq!(factorial(5), U2048::from(120 as u64));
  assert_eq!(factorial(6), U2048::from(720 as u64));
}
