// a code that counts the frequency of each number in a vector  and returns a hashmap
use std::collections::HashMap;

pub fn count_frequency(numbers: Vec<i32>) -> HashMap<i32, i32> {
    let mut frequency = HashMap::new();
    for number in numbers {
        let count = frequency.entry(number).or_insert(0);
        *count += 1;
    }
    frequency
}

