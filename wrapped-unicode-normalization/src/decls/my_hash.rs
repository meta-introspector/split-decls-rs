macro_rules! my_hash {
    () => {
        # [inline] fn my_hash (key : u32 , salt : u32 , n : usize) -> usize { let y = key . wrapping_add (salt) . wrapping_mul (2654435769) ; let y = y ^ key . wrapping_mul (0x31415926) ; (((y as u64) * (n as u64)) >> 32) as usize }
    };
}

my_hash!();