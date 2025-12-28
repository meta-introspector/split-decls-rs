macro_rules! round3_step4 {
    () => {
        # [inline] fn round3_step4 (a : & mut u32 , b : & mut u32 , c : & mut u32 , d : & mut u32 , e : & mut u32 , w : & [u32 ; 80] , t : usize ,) { * e = e . wrapping_add (w [t] . wrapping_add (a . rotate_left (5)) . wrapping_add (f3 (* b , * c , * d)) . wrapping_add (K [2]) ,) ; * b = b . rotate_left (30) ; * d = d . wrapping_add (w [t + 1] . wrapping_add (e . rotate_left (5)) . wrapping_add (f3 (* a , * b , * c)) . wrapping_add (K [2]) ,) ; * a = a . rotate_left (30) ; * c = c . wrapping_add (w [t + 2] . wrapping_add (d . rotate_left (5)) . wrapping_add (f3 (* e , * a , * b)) . wrapping_add (K [2]) ,) ; * e = e . rotate_left (30) ; * b = b . wrapping_add (w [t + 3] . wrapping_add (c . rotate_left (5)) . wrapping_add (f3 (* d , * e , * a)) . wrapping_add (K [2]) ,) ; * d = d . rotate_left (30) ; }
    };
}

round3_step4!();