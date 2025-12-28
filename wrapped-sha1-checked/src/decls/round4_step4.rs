macro_rules! round4_step4 {
    () => {
        # [inline] fn round4_step4 (a : & mut u32 , b : & mut u32 , c : & mut u32 , d : & mut u32 , e : & mut u32 , w : & [u32 ; 80] , t : usize ,) { * e = e . wrapping_add (w [t] . wrapping_add (a . rotate_left (5)) . wrapping_add (f4 (* b , * c , * d)) . wrapping_add (K [3]) ,) ; * b = b . rotate_left (30) ; * d = d . wrapping_add (w [t + 1] . wrapping_add (e . rotate_left (5)) . wrapping_add (f4 (* a , * b , * c)) . wrapping_add (K [3]) ,) ; * a = a . rotate_left (30) ; * c = c . wrapping_add (w [t + 2] . wrapping_add (d . rotate_left (5)) . wrapping_add (f4 (* e , * a , * b)) . wrapping_add (K [3]) ,) ; * e = e . rotate_left (30) ; * b = b . wrapping_add (w [t + 3] . wrapping_add (c . rotate_left (5)) . wrapping_add (f4 (* d , * e , * a)) . wrapping_add (K [3]) ,) ; * d = d . rotate_left (30) ; }
    };
}

round4_step4!();