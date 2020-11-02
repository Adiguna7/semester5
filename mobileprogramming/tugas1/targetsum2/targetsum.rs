pub fn target_sum(search: &[i32], target: i32) -> Option<(i32, i32)> {
    if search.len() < 2 {
        return None;
    }
    // For some reason hash tables are not allowed so sort the vector and
    // search from the ends.
    let mut sorted_search = search.to_vec();
    sorted_search.sort();
    let mut lowest_index = 0;
    let mut highest_index = sorted_search.len() - 1;

    while lowest_index < highest_index {
        let low = sorted_search[lowest_index];
        let high = sorted_search[highest_index];

        if low + high == target {
            return Some((low, high));
        } else if low + high > target {
            highest_index -= 1;
        } else {
            lowest_index += 1;
        }
    }
    None
}

fn main(){
    print!("{:?}", target_sum(&vec![1, 2, 5, 10], 11));
}