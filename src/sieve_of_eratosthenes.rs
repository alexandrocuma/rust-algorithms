fn sieve_of_eratosthenes(x: i64) -> bool {
  if x <= 1 {
      return false;
  }
  
  let square_root = (x as f64).sqrt() as usize;
  let mut primes = vec![true; square_root + 1];
  let mut step = 2;
  
  while step * step <= square_root {
      if primes[step] {
          for i in (step * step..=square_root).step_by(step) {
              primes[i] = false;
          }
      }
      step += 1;
  }
  
  for i in 2..=square_root {
      if primes[i] && (x as usize) % i == 0 {
          return false
      }
  }
  
  true
}

fn is_prime(x: i64) -> bool {
  let last = (x as f64).sqrt() as i64 + 1;
  x > 1 && (2..last).all(|d| x % d != 0)
}

fn is_prime(x: i64) -> bool {
  if x == 2 {return true};
  if x < 2 || x % 2 == 0 {return false};
  (3..).step_by(2).take_while(|i| i*i <= x).all(|i| x % i != 0)
}