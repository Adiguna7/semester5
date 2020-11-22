const SPACE: u8 = b' ';
#[inline]
pub fn ascii_reverse_words(s: &mut String) {
    let len = s.len();
    if len == 0 {
        return;
    }

    if !s.is_ascii() {        
        panic!("Unexpected non-ASCII string: \"{}\"", s);
    }

    unsafe {
        let ref mut bytes = s.as_mut_vec();  // Unsafe.        
        bytes.reverse();
        
        let mut left = 0;
        while left < len {
            if bytes[left] == SPACE {
                left += 1;
            } else {
                let mut right = left;
                while right < len && bytes[right] != SPACE {
                    right += 1;
                }
                bytes[left..right].reverse();
                left = right;
            }
        }
    }
}

fn main(){
    let mut s = "Hello World".to_string();
    ascii_reverse_words(&mut s);
    print!("{}", s);

}