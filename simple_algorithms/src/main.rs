#![allow(unused_imports)]
mod un_ordered_array;
use rand::prelude::*;
use un_ordered_array::UnOrderedArray;

fn main() {
    let mut ua: UnOrderedArray = UnOrderedArray::new();
    let mut rgn = thread_rng();

    for _ in 0..10{
        ua.add_last(rgn.gen_range(0..100))
    }

    println!("{}", &ua.to_string());
    ua.selection_sort();

    println!("{}", &ua.to_string());
}
