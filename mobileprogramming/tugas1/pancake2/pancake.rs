#[inline]
fn reverse_until_index<T>(v: &mut [T], index: usize) {
    v[..index+1].reverse();
}
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
#[inline]
pub fn pancake_sort<T>(v: &mut [T]) where T: PartialOrd {
    if v.len() == 0 {
        return;
    }
    let mut index = v.len() - 1;

    while index > 0 {        
        let max_index = index_of_max(&v, index);
        if max_index != index {            
            reverse_until_index(&mut *v, max_index);            
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