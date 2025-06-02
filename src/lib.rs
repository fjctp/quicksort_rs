#[allow(dead_code)]
/// Sorts the given mutable slice in-place using the quicksort algorithm.
///
/// This function operates recursively, choosing the last element as the pivot,
/// partitioning the array around it, and then sorting the subarrays to the left
/// and right of the pivot.
///
/// # Type Parameters
///
/// - `T`: The element type of the slice. Must implement `Ord` for comparisons.
///
/// # Arguments
///
/// - `arr`: A mutable slice of elements to be sorted.
///
/// # Example
///
/// ```
/// let mut data = vec![3, 1, 4, 1, 5, 9];
/// quicksort(&mut data);
/// assert_eq!(data, vec![1, 1, 3, 4, 5, 9]);
/// ```
pub fn quicksort<T: Ord>(arr: &mut [T]) {
    // Base case: a slice with 0 or 1 elements is already sorted
    if arr.len() <= 1 {
        return;
    }

    // Partition the slice and get the index where the pivot is placed
    // - left of pivot: all elements less than or equal to the pivot
    // - right of pivot: the pivot itself + elements greater than the pivot
    let pivot_index = partition(arr);

    // Split the slice into two parts:
    let (left, right) = arr.split_at_mut(pivot_index);

    // Recursively sort left and right.
    quicksort(left);
    quicksort(&mut right[1..]); // skipping the pivot itself (already in place)
}

/// Partitions a mutable slice around a pivot element chosen as the last element.
///
/// Elements less than or equal to the pivot are moved to the left,
/// and elements greater than the pivot are moved to the right.
/// The pivot is placed at its final sorted position, and its index is returned.
///
/// # Type Parameters
///
/// - `T`: The element type of the slice. Must implement `Ord`.
///
/// # Arguments
///
/// - `arr`: A mutable slice to be partitioned.
///
/// # Returns
///
/// - The index where the pivot element ends up.
///
/// # Panics
///
/// - If the input slice is empty. This function assumes `arr.len() >= 1`.
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let mut i = 0; // i marks the boundary of elements <= pivot

    // Loop through all elements except the pivot (last element)
    for j in 0..len - 1 {
        // If current element is <= pivot, swap it to the front
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    // Place the pivot in its correct sorted position
    arr.swap(i, len - 1);
    
    // Return the pivot's final index
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Create test data
        let unsorted = vec![33, 10, 55, 71, 29, 3, 18];

        // Generate test value
        let mut uut = unsorted.clone();
        quicksort(&mut uut);

        // Generate expected value
        let mut exp = unsorted.clone();
        exp.sort();
        
        // Check values
        println!("Original: {:?}", unsorted);
        println!("Sorted:   {:?}", uut);
        assert_eq!(uut, exp);
    }
}
