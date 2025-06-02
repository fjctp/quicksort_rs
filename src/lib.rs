#[allow(dead_code)]
pub fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left);
    quicksort(&mut right[1..]); // skip the pivot
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let unsorted = vec![33, 10, 55, 71, 29, 3, 18];
        let mut uut = unsorted.clone();
        quicksort(&mut uut);

        let mut exp = unsorted.clone();
        exp.sort();

        println!("Original: {:?}", unsorted);
        println!("Sorted:   {:?}", uut);
        assert_eq!(uut, exp);
    }
}
