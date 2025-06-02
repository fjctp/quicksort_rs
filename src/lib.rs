#[allow(dead_code)]
pub fn quicksort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr[0];
    let less: Vec<i32> = arr.iter().skip(1).filter(|&&x| x <= pivot).cloned().collect();
    let greater: Vec<i32> = arr.iter().skip(1).filter(|&&x| x > pivot).cloned().collect();

    let mut sorted = quicksort(less);
    sorted.push(pivot);
    sorted.extend(quicksort(greater));
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let unsorted = vec![33, 10, 55, 71, 29, 3, 18];
        let uut = quicksort(unsorted.clone());

        let mut exp = unsorted.clone();
        exp.sort();

        println!("Original: {:?}", unsorted);
        println!("Sorted:   {:?}", uut);
        assert_eq!(uut, exp);
    }
}
