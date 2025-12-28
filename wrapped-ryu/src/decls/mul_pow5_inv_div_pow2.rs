macro_rules! mul_pow5_inv_div_pow2 {
    () => {
        # [cfg_attr (feature = "no-panic" , inline)] pub fn mul_pow5_inv_div_pow2 (m : u32 , q : u32 , j : i32) -> u32 { # [cfg (feature = "small")] { let pow5 = unsafe { d2s :: compute_inv_pow5 (q) } ; mul_shift_32 (m , pow5 . 1 + 1 , j) } # [cfg (not (feature = "small"))] { debug_assert ! (q < d2s :: DOUBLE_POW5_INV_SPLIT . len () as u32) ; unsafe { mul_shift_32 (m , d2s :: DOUBLE_POW5_INV_SPLIT . get_unchecked (q as usize) . 1 + 1 , j ,) } } }
    };
}

mul_pow5_inv_div_pow2!()