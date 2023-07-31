fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
  let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<_>>(); 
  let mut a2 = b;
  a1.sort();
  a2.sort();
  a1 == a2
}