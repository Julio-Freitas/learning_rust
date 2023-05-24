//iter().map(|item| item * 1).rev().collect::<Vec<i32>>()

pub fn sort_array<T: Ord>(array: &mut [T]) -> &mut [T] {
    for i in 0..array.len() {
        for j in ((i + 1)..array.len()).rev() {
            if array[j - 1] > array[j] {
                array.swap(j - 1, j);
            }
        }
    }
    return array;
}
