use crate::bits::*;
use rand::random;
pub fn test_ones_except() {
    for i in 0..64 {
        print!("f({:02}) = {:064b}\n",i, ones_except(i));
    }
}
pub fn test_zeros_except() {
    for i in 0..64 {
        print!("f({:02}) = {:064b}\n",i, zeros_except(i));
    }
}
pub fn test_trident() {
    //let x = u64::MAX/8723;
    let x = ones_except(34);
    print!("x     = {:064b}\n", x);
    for i in 0..64 {
        print!("f({:02}) = {:064b}\n",i, trident(x,i));
    }
}
pub fn test_wings() {
    let x = u64::MAX/8723;
    //let x = ones_except(34);
    print!("x     = {:064b}\n", x);
    for i in 0..64 {
        print!("f({:02}) = {:064b}\n",i, wings(x,i));
    }
}
pub fn test_eval() {
    //let x = u64::MAX/8723;
    let x = random::<u64>();
    //let x = ones_except(34);
    print!("x     = {:064b}\n", x);
    for i in 0..64 {
        print!("f({:02}) = {:064b}\n",i, eval(x,i));
    }
}

pub fn print_bits(s : u64) {
    for i in 0..64 {
        if eval(s,i) == 1 {print!("\u{2588}");} else {print!(" ");}
    }
    print!("\n");
}
pub fn _prnt(s : u64, i : u32) { if eval(s,i) == 1 {print!("\u{2588}");} else {print!(" ");}