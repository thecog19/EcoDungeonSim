extern crate rand;

use rand::Rng;
fn main() {
  let seed: i32 = generate_seed();
  build_dungeon(seed);
}


fn generate_seed() -> i32{
    println!("Generating a random seed for dungeon generation");
let seed: i32 = rand::thread_rng().gen_range(10000000,99999999);
println!("Seed is: {}", seed);
    return seed
}



fn build_dungeon(seed: i32){
    let dungeon: Vec<Vec<i32>>;
    let digits: Vec<_> = seed.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    println!("{}", digits[0])
   // print_dungeon(dungeon);
}

fn print_dungeon(dungeon:  Vec<Vec<i32>> ){
    for x in dungeon.iter(){
        println!("{:?}", x);
    }
}
