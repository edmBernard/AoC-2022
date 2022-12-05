#[derive(Debug, PartialEq)]
pub enum ReturnType {
  Numeric(u64, u64),
  String(String, String),
}

impl PartialEq<[u64; 2]> for ReturnType {
  fn eq(&self, other: &[u64; 2]) -> bool {
    match self {
      ReturnType::Numeric(part1, part2) => [*part1, *part2] == *other,
      ReturnType::String(_, _) => {
        panic!("Should not compare ReturnType::String with [u64; 2]");
      }
    }
  }
}

impl PartialEq<[&str; 2]> for ReturnType {
  fn eq(&self, other: &[&str; 2]) -> bool {
    match self {
      ReturnType::Numeric(_, _) => {
        // Best way I found, I would have prefer a compilation error but I don't found how
        panic!("Should not compare ReturnType::Numeric with [&str; 2]");
      }
      ReturnType::String(part1, part2) => [&part1[..], &part2[..]] == *other,
    }
  }
}
