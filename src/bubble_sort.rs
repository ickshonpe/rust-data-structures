pub fn sort<T: PartialOrd>(data: &mut [T]) {
    if data.len() > 1 {
        let mut finished = false;
        while !finished {
            finished = true;
            for i in 1..data.len() {
                if data[i - 1] > data[i] {
                    data.swap(i - 1, i);
                    finished = false;
                }
            }
        }
    }
}

pub fn example() {
    let mut xs = (0..10).rev().collect::<Vec<u64>>();
    println!("Bubble sort");
    println!("Before ->\t {:?}", xs );
    sort(&mut xs);
    println!("After ->\t {:?}", xs);
}