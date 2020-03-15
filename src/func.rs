use numext_fixed_uint::U2048;

//nPr
pub fn permutation(n64: u64, r: u64) -> U2048 {
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
pub fn combin(n64: u64, r: u64) -> U2048 {
  let n2: U2048 = permutation(n64, r).into();
  let r2: U2048 = permutation(r, r).into();
  let a = n2 / r2;
  //println!("combin {}C{} -> {}", n64, r, a.clone());
  a
}

//n!
pub fn factorial(n: u64) -> U2048 {
  let mut m: U2048 = U2048::from(1 as u64);
  for i_64 in 1..(n + 1) {
    let i: U2048 = i_64.into();
    m = m * i;
    //println!("m : {}", m)
  }
  //println!("factorial : {}! -> {}", n, m);
  m
}

pub fn number_of_combinations(all: u64, group: u64) -> U2048 {
  // (400C50 * 350C50 * 300C50 * 250C50 * 200C50 * 150C8 * 100C50) / 50!
  let member = all / group;
  let mut n: U2048 = U2048::from(1 as u64);
  for i in 0..group {
    let nokori = all - member * i;
    n = n * combin(nokori, member);
    //println!("n : {}", n);
  }
  //println!("main : {}", n);
  let l = factorial(group);
  n / l
}

fn get_size(n: U2048) -> (f32, usize) {
  let n_str = format!("{}", n);
  let n_size = n_str.len() - 1;
  let n_num: f32 = format!(
    "{}.{}",
    n_str.chars().nth(0).unwrap(),
    n_str.chars().nth(1).unwrap()
  )
  .parse()
  .unwrap();
  (n_num, n_size)
}

pub fn check_size(all: u64, group: u64, n: u64) -> String {
  let member = all / group;
  //350C48
  let (com_num, com_size) = get_size(combin(all - member, member - n - 1));
  println!(
    "{}C{} -> {} * (10 ^ {})",
    all - member,
    member - n - 1,
    com_num,
    com_size
  );
  // f 350 7
  let (main_num, main_size) = get_size(number_of_combinations(all - member, group - 1));
  println!(
    "f {} {} -> {} * (10 ^ {})",
    all - member,
    group - 1,
    main_num,
    main_size
  );
  // 49C1
  let (m_num, m_size) = get_size(combin(member - 1, n));
  println!("{}C{} -> {} * (10 ^ {})", member - 1, n, m_num, m_size);
  let all_num = com_num * main_num * m_num;
  let all_size = com_size + main_size + m_size;
  format!("{} * (10 ^ {})", all_num, all_size)
}

#[test]

fn check() {
  assert_eq!(combin(4, 2), U2048::from(6 as u64));
  assert_eq!(combin(6, 2), U2048::from(15 as u64));
  //assert_eq!(combin(9, 3), U2048::from(280 as u64));

  assert_eq!(factorial(5), U2048::from(120 as u64));
  assert_eq!(factorial(6), U2048::from(720 as u64));

  let (c1_f, c1_n) = get_size(U2048::from(256 as u64));
  assert_eq!(
    format!("{} * (10 ^ {})", c1_f, c1_n),
    "2.5 * (10 ^ 2)".to_string()
  );

  let (c2_f, c2_n) = get_size(U2048::from(256000000000000 as u64));
  assert_eq!(
    format!("{} * (10 ^ {})", c2_f, c2_n),
    "2.5 * (10 ^ 14)".to_string()
  );
}
