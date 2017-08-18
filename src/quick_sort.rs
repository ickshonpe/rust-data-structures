
pub fn quick_sort<T: PartialOrd>(data: &mut [T]) {
    fn r<T: PartialOrd>(data: &mut [T], start: usize, end: usize) {
        if start < end {
            let p = partition(data, start, end);
            if p != 0 && start < p - 1 {
                r(data, start, p - 1);
            }
            if start < p + 1 {
                r(data, p + 1, end);
            }
        }
    }
    let len = data.len();
    r(data, 0, len - 1)
}

fn partition<T: PartialOrd>(data: &mut [T], start: usize, end: usize) -> usize {
    let mut i = start;
    for j in start..end {
        if data[j] < data[end] {
            i += 1;
            data.swap(i - 1, j);
        }
    }
    if data[end] < data[i] {
        data.swap(i, end);
    }
    i
}

pub fn example() {
    let mut ns = [9, 8, 7, 6, 4, 5, 3, 2, 1];
    print!("input ->\t");
    println!("{:?}", ns);
    quick_sort(&mut ns);
    print!("Sorted! ->\t");
    println!("{:?}", ns);
}

#[test]
fn test_1() {
    let mut xs: Vec<i64> = (0..1_000).collect();
    quick_sort(&mut xs);
    for (i, x) in xs.iter().enumerate() {
        assert!(i == *x as usize);
    }
}

#[test]
pub fn test_2() {
    let mut xs: Vec<i64> = (0..1_000).rev().collect();
    quick_sort(&mut xs);
    for (i, x) in xs.iter().enumerate() {
        assert!(i == *x as usize);
    }

}