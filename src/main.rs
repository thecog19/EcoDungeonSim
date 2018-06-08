extern crate rand;

struct Dungeon {
    map: Vec<Vec<i32>>,
    seed: Vec<u32>,
}

// pub struct Node<T> {
//     parent: Option<NodeId>,
//     previous_sibling: Option<NodeId>,
//     next_sibling: Option<NodeId>,
//     first_child: Option<NodeId>,
//     last_child: Option<NodeId>,

//     /// The actual data which will be stored within the tree
//     pub data: T,
// }

// pub struct NodeId {
//     index: usize,
// }

use rand::Rng;
fn main() {
    let seed: i32 = generate_seed();
    build_dungeon(seed);
}

fn generate_seed() -> i32 {
    println!("Generating a random seed for dungeon generation");
    let seed: i32 = rand::thread_rng().gen_range(10000000, 99999999);
    println!("Seed is: {}", seed);
    return seed;
}

fn build_dungeon(seed: i32) {
    let digits: Vec<u32> = seed.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    //let dungeon = Dungeon { map: seed: digits };
    //the first character of the string is the default number of rooms
    // now we generate
    // print_dungeon(dungeon);
}

fn assemble_layout(rooms: i32, seed: i32) {}

fn print_dungeon(dungeon: Vec<Vec<i32>>) {
    for x in dungeon.iter() {
        println!("{:?}", x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_seed_is_random() {
        let first_try = generate_seed();
        let second_try = generate_seed();
        assert_ne!(first_try, second_try,
        "Seed generator isn't random enough.")
    }
}
