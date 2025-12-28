macro_rules! round4_step_bw {
    () => {
        # [inline (always)] fn round4_step_bw (a : u32 , b : & mut u32 , c : u32 , d : u32 , e : & mut u32 , mt : u32) { * b = b . rotate_right (30) ; * e = e . wrapping_sub (a . rotate_left (5) . wrapping_add (f4 (* b , c , d)) . wrapping_add (K [3]) . wrapping_add (mt) ,) ; }
    };
}

round4_step_bw!();