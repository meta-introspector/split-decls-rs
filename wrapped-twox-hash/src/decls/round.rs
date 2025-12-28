macro_rules! round {
    () => {
        # [inline] const fn round (mut acc : u64 , lane : u64) -> u64 { acc = acc . wrapping_add (lane . wrapping_mul (PRIME64_2)) ; acc = acc . rotate_left (31) ; acc . wrapping_mul (PRIME64_1) }
    };
}

round!()