use std::fmt::{self, Display, Formatter, Result};

struct Watermelon {
  color: &'static str, 
  seeds: u16, 

  // u8 is 8 bits, each bit has 2 possibilities
  // 2^8 = 256 possibly not enough values to respresent total seeds in a watermelon
  // so ... u16 is 16 bits, each bit has 2 possibilities
  // 2 ^ 16 = 65, 536 ( definietly enough for our watermelon)
  // unsigned interger becuase seed count is never integer. 

}

impl Display for Watermelon {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "(color:{}, seeds:{})", self.color, self.seeds)
  }
}


fn main() {
  let mut bucket = [
    Watermelon { color: "red", seeds: 24 },
    Watermelon { color: "blue", seeds: 56 },
    Watermelon { color: "green", seeds: 106 }
  ];

// bucket[3] = Watermelon { color: "yellow", seeds: 30};

  println! ("{}", bucket[0]);
 
}




