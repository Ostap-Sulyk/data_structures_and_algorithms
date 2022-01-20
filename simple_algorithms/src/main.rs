#![allow(unused_imports)]
mod un_ordered_array;
use std::time::{Duration, Instant};

use rand::prelude::*;

use un_ordered_array::UnOrderedArray;

fn main() {
    let mut ua: UnOrderedArray = UnOrderedArray::new();
    let mut rgn = thread_rng();

    for _ in 0..10 {
        ua.add_last(rgn.gen_range(0..100))
    }

    // let now = Instant::now();
    println!("{}", ua.list_items());
    ua.selection_sort();
    println!("{}", ua.list_items());
    ua.selection_sort_desc();
    println!("{}", ua.list_items());

    // println!("{}", now.elapsed().as_secs());
}
