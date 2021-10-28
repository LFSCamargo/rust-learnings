pub fn hello() -> String {
  ("Hello, world!").to_string()
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::hello;

  #[test]
  fn test_main() {
    assert_eq!(hello(), "Hello, world!");
  }
}
