use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec;
use std::collections::VecDeque;

pub mod logic;
pub use logic::count_frequency;


fn main() {
 
    let mut fruit = vec!["apple", "banana", "cherry", "blueberry", "orange", "kiwi", "mango"];
    let mut i = 0;
    while i < fruit.len() {
        println!("{}", fruit[i]);
        i += 1;
    }

    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    println!("Shuffled fruits:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let frequency = count_frequency(numbers);
    println!("{:?}", frequency);
    //let mut fruit_queue = VecDeque::new();

}
