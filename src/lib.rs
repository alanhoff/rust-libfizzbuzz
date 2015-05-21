// Detect if an i64 is divisible by 3
pub fn is_fizz(n: i64) -> bool {
  n % 3 == 0
}

// Detect if an i64 is divisible by 5
pub fn is_buzz(n: i64) -> bool {
  (n % 5) == 0
}

// Detect if an i64 is divisible by 3 and 5
pub fn is_fizzbuzz(n: i64) -> bool {
  (n % 5) == 0 && (n % 3) == 0
}

// Detect if an i64 is fizz, buzz and fizzbuzz
pub fn detect(n: i64) -> (bool, bool, bool) {
  let fizz = is_fizz(n);
  let buzz = is_buzz(n);
  let fizzbuzz = is_fizzbuzz(n);

  (fizz, buzz, fizzbuzz)
}
