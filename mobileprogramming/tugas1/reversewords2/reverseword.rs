// use std::ascii::AsciiExt;

const SPACE: u8 = b' ';

/// Reverse the words in the input string. Words are defined as any characters
/// other than a space. Only ASCII input is acceptable.
///
/// # Examples
///
/// ```
/// use reversewords::ascii_reverse_words;
///
/// let mut s = "Hello from Rust!".to_string();
/// ascii_reverse_words(&mut s);
/// assert_eq!("Rust! from Hello", s)
/// ```
#[inline]
pub fn ascii_reverse_words(s: &mut String) {
    let len = s.len();
    if len == 0 {
        return;
    }

    if !s.is_ascii() {
        // Reversing non-ASCII strings is hard because you have to make sure
        // not to spit graphemes.
        panic!("Unexpected non-ASCII string: \"{}\"", s);
    }

    unsafe {
        let ref mut bytes = s.as_mut_vec();  // Unsafe.
        // Reverse the entire string. So:
        // "Hello from Rust!" => "!tsuR morf olleH"
        bytes.reverse();

        // Find each "word" (non-space) in `bytes` and reverse it. So
        // "!tsuR" => "Rust1".
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