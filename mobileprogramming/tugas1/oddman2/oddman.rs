pub fn odd_man_out(l: &[i32]) -> i32 {
    let mut mask : i32 = 0;
    for x in l {
        // Take advantage of the fact that xor-ing an integer with a value
        // twice results in the original integer. So every value except for
        // the unpaired integer should have cancelled itself out. Then take
        // advantage of the property that x^0 == x.
        mask ^= *x;
    }
    return mask;
}

fn main(){
    print!("{}", odd_man_out(&[1, 2, 3, 4, 5, 1, 2, 3, 4]));
}