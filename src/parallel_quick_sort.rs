use rayon;
use std;

//  stack overflows on larger inputs

pub fn quick_sort<T: PartialOrd + std::marker::Send>(data: &mut [T]) {
    let len = data.len();
    if len > 1 {
        let p = partition(data);
        let (less, more) = split_at_exclusive_mut(data, p);
        rayon::join(
            || { quick_sort(less) },
            || { quick_sort(more) }
        );
    }
}

pub fn split_at_exclusive_mut<T>(data: &mut [T], p: usize) -> (&mut [T], &mut [T]) {
    let (front, back) = data.split_at_mut(p);
    let back = &mut back[1..];
    (front, back)
}

fn partition<T: PartialOrd>(data: &mut [T]) -> usize {
    let mut cursor = 0;
    let pivot_index = data.len() - 1;
    for j in 0..data.len() {
        if data[j] < data[pivot_index] {
            data.swap(cursor, j);
            cursor += 1;
        }
    }
    if data[pivot_index] < data[cursor] {
        data.swap(cursor, pivot_index);
    }
    cursor
}

pub fn example() {
    let mut ns = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    print!("input ->\t");
    println!("{:?}", ns);
    quick_sort(&mut ns);
    print!("Sorted! ->\t");
    println!("{:?}", ns);
}

#[test]
fn test_1() {
    for r in 1..100 {
        let mut xs = (0..r).collect::<Vec<i64>>();
        quick_sort(&mut xs);
        for (i, x) in xs.iter().enumerate() {
            assert!(i == *x as usize);
        }
    }
}

#[test]
pub fn test_2() {
    for r in 1..100 {
        let mut xs = (0..r).rev().collect::<Vec<i64>>();
        quick_sort(&mut xs);
        for (i, x) in xs.iter().enumerate() {
            assert!(i == *x as usize);
        }
    }
}