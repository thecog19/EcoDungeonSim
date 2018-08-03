extern crate rand;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};


fn main() {
  let seed: i64 = generate_seed();
  println!("{:?}", seed);
  
}

use rand::Rng;
fn generate_seed() -> i64 {
  return rand::thread_rng().gen_range(1_000_000_000, 10_000_000_000);
}