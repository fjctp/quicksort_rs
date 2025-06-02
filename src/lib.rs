#[allow(dead_code)]
pub fn quicksort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr.remove(0);
    let mut less: Vec<i32> = Vec::new();
    let mut greater: Vec<i32> = Vec::new();

    for x in arr {
        if x <= pivot {
            less.push(x);
        } else {
            greater.push(x);
        }
    }

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
