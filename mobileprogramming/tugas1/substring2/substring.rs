pub fn has_substring(search: &str, find: &str) -> bool {    
    if search.is_empty() {
        return find.is_empty();
    }

    let search_bytes = search.as_bytes();
    let find_bytes = find.as_bytes();
    let max_search_index = search_bytes.len() - find_bytes.len();

    for i in 0..(max_search_index + 1) {
        let mut found = true;
        for j in 0..find_bytes.len() {
            if search_bytes[i + j] != find_bytes[j] {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}

fn main(){
    print!("{}",has_substring("abcdef", "def"));
}