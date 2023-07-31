fn rot13(message: &str) -> String {
  message.chars().map(|letter| {
      let new_letter = match letter {
          'a'..='z' => (((letter as u8) - ('a' as u8) + 13) % 26) + ('a' as u8),
          'A'..='Z' => (((letter as u8) - ('A' as u8) + 13) % 26) + ('A' as u8),
          _ => letter as u8
      };
      new_letter as char
  }).collect()
}

fn rot13(message: &str) -> String {
  message.chars().map(|c| {
      match c {
          'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
          'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
          _ => c,
      }
  }).collect()
}