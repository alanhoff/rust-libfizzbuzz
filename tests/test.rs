extern crate libfizzbuzz;
use libfizzbuzz as lfb;

#[test]
fn test_is_fizz() {
  assert_eq!(true, lfb::is_fizz(3));
  assert_eq!(false, lfb::is_fizz(5));
}

#[test]
fn test_is_buzz() {
  assert_eq!(true, lfb::is_buzz(5));
  assert_eq!(false, lfb::is_buzz(6));
}

#[test]
fn test_is_fizzbuzz() {
  assert_eq!(true, lfb::is_fizzbuzz(15));
  assert_eq!(false, lfb::is_fizzbuzz(5));
  assert_eq!(false, lfb::is_fizzbuzz(3));
}

#[test]
fn test_detect() {
  let (fizz, buzz, fizzbuzz) = lfb::detect(15);
  assert_eq!(true, fizz);
  assert_eq!(true, buzz);
  assert_eq!(true, fizzbuzz);
}
