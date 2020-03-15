use bigint::U128;
use bigint::U256;
use bigint::U512;

pub mod func;

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
  println!(
    "Length of combin : {}",
    format!("{}", func::combin(49, 1)).len() - 1
  );
  println!(
    "Length of combin : {}",
    format!("{}", func::combin(350, 48)).len() - 1
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

fn main() {
  //check_main();
  //check_1();
  check_2();
}
