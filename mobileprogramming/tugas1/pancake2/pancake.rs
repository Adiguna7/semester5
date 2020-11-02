/// Reverse the items in the list from [0..index].
#[inline]
fn reverse_until_index<T>(v: &mut [T], index: usize) {
    v[..index+1].reverse();
}

/// Return the index of the largest item in the Vec between [0..index].
#[inline]
fn index_of_max<T>(v: &[T], index: usize) -> usize where T: PartialOrd {
    let mut max = &v[0];
    let mut max_index = 0;
    for i in 0..(index+1) {
        if v[i] > *max {
            max = &v[i];
            max_index = i;
        }
    }
    max_index
}

/// Sort the Vec using only `reverse_until_index` to exchange elements.
/// See: http://en.wikipedia.org/wiki/Pancake_sorting
///
/// # Examples
///
/// ```
/// use pancake::pancake_sort;
///
/// let mut v = vec![5, 1, 3, 4, 8, 2, 1, 9, 7];
/// pancake_sort(&mut v);
/// assert_eq!(vec![1, 1, 2, 3, 4, 5, 7, 8, 9], v);
/// ```
#[inline]
pub fn pancake_sort<T>(v: &mut [T]) where T: PartialOrd {
    if v.len() == 0 {
        return;
    }
    let mut index = v.len() - 1;

    while index > 0 {
        // Only consider items in the range [0..index], which have yet to be
        // sorted.
        let max_index = index_of_max(&v, index);
        if max_index != index {
            // Move the largest item to the 0th position.
            reverse_until_index(&mut *v, max_index);
            // Reverse the Vec so the 0th item is now at `index`. 
            reverse_until_index(&mut *v, index);
        }
        index -= 1;
    }
}


fn main(){
    let mut v = vec![5, 4, 3, 2, 1];
    pancake_sort(&mut v);
    print!("{:?}", v);    
}