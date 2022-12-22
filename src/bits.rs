use rand::random;


pub fn ones_except(i : u32) -> u64 {u64::MAX ^ (1u64.rotate_left(i))}
pub fn zeros_except(i : u32) -> u64 { 1u64.rotate_left(i)}