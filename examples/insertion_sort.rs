extern crate algorithms;

use algorithms::sorts::insertion_sort;

fn main() {
    let to_sort = vec![0, -5, 5, 1, 14];
    let sorted = insertion_sort::sort(&to_sort);
    println!("Sorted: {:?}", &sorted);

    let mut to_sort = vec![0, -5, 5, 1, 14];
    insertion_sort::mut_sort(&mut to_sort);
    println!("Sorted: {:?}", &to_sort);
}
