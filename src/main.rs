
mod list;
use list::*;


fn main() {
    let mut list = List::new();
    for i in 0..100_000_000 {
        list.push(i);
    }
    println!("{}", list.len());
}