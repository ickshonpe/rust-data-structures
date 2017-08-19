extern crate rayon;

mod merge_sort;
mod tricks;
mod list;
mod quick_sort;
mod quick_sort_slice;
mod parallel_quick_sort;
mod bubble_sort;

use list::*;


fn main() {
    quick_sort::example();
    println!();
    bubble_sort::example();
}