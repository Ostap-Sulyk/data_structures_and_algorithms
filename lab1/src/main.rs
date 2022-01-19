mod un_ordered_array;
use un_ordered_array::UnOrderedArray;
use rand::prelude::*;

fn main() {
    let mut ua: UnOrderedArray = UnOrderedArray::new();
    let mut rgn = thread_rng();

    for _ in 0..1000000 {
        ua.add_last(rgn.gen_range(0..1000000))
    };
}
