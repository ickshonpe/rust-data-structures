
pub fn merge_sort<T: PartialOrd + Copy>(xs: &[T]) -> Vec<T> {
    if xs.len() > 1 {
        let pivot = xs.len() / 2;
        let (left, right) = xs.split_at(pivot);
        let left = merge_sort(left);
        let right = merge_sort(right);
        merge(&left, &right)
    } else {
        xs.to_vec()
    }
}

pub fn merge<T: PartialOrd + Copy>(mut left: &[T], mut right: &[T]) -> Vec<T> {
    let mut zs: Vec<T> = Vec::with_capacity(left.len() + right.len());
    loop {
        if left.is_empty() {
            for &element in right {
                zs.push(element);
            }
            break
        }
        if right.is_empty() {
            for &element in left {
                zs.push(element);
            }
            break
        }
        if left[0] < right[0] {
            zs.push(left[0]);
            left = &left[1..];
        } else {
            zs.push(right[0]);
            right = &right[1..];
        }
    }
    zs
}

