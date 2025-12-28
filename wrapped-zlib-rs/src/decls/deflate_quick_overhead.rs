macro_rules! deflate_quick_overhead {
    () => {
        const fn deflate_quick_overhead (x : usize) -> usize { let sum = x . wrapping_mul (DEFLATE_QUICK_LIT_MAX_BITS - 8) . wrapping_add (7) ; (sum as core :: ffi :: c_ulong >> 3) as usize }
    };
}

deflate_quick_overhead!();