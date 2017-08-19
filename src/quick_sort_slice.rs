// improved quicksort using slices
// pivots on the last element of the array
// todo: implement without recursion

pub fn quick_sort<T: PartialOrd>(data: &mut [T]) {
    if data.len() > 1 {
        let p = partition(data);
        quick_sort(&mut data[..p]);
        quick_sort(&mut data[p + 1..]);
    }
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
    let mut xs = (0..1_000).collect::<Vec<i64>>();
    quick_sort(&mut xs);
    for (i, x) in xs.iter().enumerate() {
        assert!(i == *x as usize);
    }
}

#[test]
pub fn test_2() {
    let mut xs = (0..1_000).rev().collect::<Vec<i64>>();
    quick_sort(&mut xs);
    for (i, x) in xs.iter().enumerate() {
        assert!(i == *x as usize);
    }

}