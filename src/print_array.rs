fn printer_error(s: &str) -> String {
  let mut errors = 0;
  s.chars().for_each(|x| {
      match x {
          'n'..='z' => errors += 1,
          _ => ()
      }
  });
  format!("{}/{}", errors, s.len())
}

fn printer_error(s: &str) -> String {
  // Your cude here
  format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
}