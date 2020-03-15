pub mod func;

fn check(all: u64, cls: u64, n: u64) {
  println!(
    "{}人を{}クラスにクラス替えするときに、再び同じクラスになる人が{}人の時の組み合わせ総数:\n{}\n",
    all,
    cls,
    n,
    func::check_size(all, cls, n)
  )
}

fn check_main() {
  println!(
    "Number of number_of_combinations : {}",
    func::number_of_combinations(400, 8)
  );
  println!(
    "Length of number_of_combinations : {}",
    format!("{}", func::number_of_combinations(400, 8)).len() - 1
  );
}

fn check_1() {
  println!(
    "Length of combin : {}",
    format!("{}", func::combin(350, 49)).len() - 1
  );
  println!(
    "Number of number_of_combinations : {}",
    func::number_of_combinations(350, 7)
  );
  println!(
    "Length of number_of_combinations : {}",
    format!("{}", func::number_of_combinations(350, 7)).len() - 1
  );
}

fn check_2() {
  //  println!(
  //    "Length of combin : {}",
  //    format!("{}", func::combin(49, 1)).len() - 1
  //  );
  //  println!(
  //    "Length of combin : {}",
  //    format!("{}", func::combin(350, 48)).len() - 1
  //  );
  //  println!(
  //    "Number of number_of_combinations : {}",
  //    func::number_of_combinations(350, 7)
  //  );
  //  println!(
  //    "Length of number_of_combinations : {}",
  //    format!("{}", func::number_of_combinations(350, 7)).len() - 1
  //  );
  println!("{}", func::check_size(400, 8, 1))
}

fn main() {
  //check_main();
  //check_1();
  //check_2();
  check(400, 8, 1);
  check(400, 8, 2);
  check(400, 8, 3);
  check(400, 8, 4);
  check(400, 8, 5);
  check(400, 8, 6);
  check(400, 8, 7);
  check(400, 8, 8);
  check(400, 8, 9);
  check(400, 8, 10);
  check(400, 8, 11);
  check(400, 8, 12);
  check(400, 8, 13);
  check(400, 8, 14);
  check(400, 8, 15);
  check(400, 8, 16);
  check(400, 8, 17);
  check(400, 8, 18);
  check(400, 8, 19);
  check(400, 8, 20);
  check(400, 8, 21);
  check(400, 8, 22);
  check(400, 8, 23);
  check(400, 8, 24);
  check(400, 8, 25);
  check(400, 8, 26);
  check(400, 8, 27);
  check(400, 8, 28);
  check(400, 8, 29);
  check(400, 8, 30);
  check(400, 8, 31);
  check(400, 8, 32);
  check(400, 8, 33);
  check(400, 8, 34);
  check(400, 8, 35);
  check(400, 8, 36);
  check(400, 8, 37);
  check(400, 8, 38);
  check(400, 8, 39);
  check(400, 8, 40);
  check(400, 8, 41);
  check(400, 8, 42);
  check(400, 8, 43);
  check(400, 8, 44);
  check(400, 8, 45);
  check(400, 8, 46);
  check(400, 8, 47);
  check(400, 8, 48);
  check(400, 8, 49);
}
