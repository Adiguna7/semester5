pub fn odd_man_out(l: &[i32]) -> i32 {
    let mut mask : i32 = 0;
    for x in l {        
        mask ^= *x;
    }
    return mask;
}

fn main(){
    print!("{}", odd_man_out(&[1, 2, 3, 4, 5, 1, 2, 3, 4]));
}