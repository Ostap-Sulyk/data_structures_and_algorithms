#![allow(unused_imports)]
mod un_ordered_array;
use std::time::{Duration, Instant};

use rand::prelude::*;

use un_ordered_array::UnOrderedArray;

fn main() {
    let mut ua1: UnOrderedArray = UnOrderedArray::new();
    let mut ua2: UnOrderedArray = UnOrderedArray::new();

    let mut rgn = thread_rng();
    for _ in 0..100_000 {
        let num = rgn.gen_range(0..100_000);
        ua1.add_last(num);
        ua2.add_last(num);
    }

    println!("Finished generating numbers");

    let now = Instant::now();
    ua1.selection_sort();
    println!("selection sort took: {}", now.elapsed().as_millis());

    let now = Instant::now();
    ua2.insertion_sort();
    println!("insertion sort took: {}", now.elapsed().as_millis());
}
