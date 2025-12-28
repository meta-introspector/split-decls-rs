macro_rules! round1_step4 {
    () => {
        # [inline (always)] fn round1_step4 (a : & mut u32 , b : & mut u32 , c : & mut u32 , d : & mut u32 , e : & mut u32 , w : & [u32 ; 80] , t : usize ,) { * e = e . wrapping_add (w [t] . wrapping_add (a . rotate_left (5)) . wrapping_add (f1 (* b , * c , * d)) . wrapping_add (K [0]) ,) ; * b = b . rotate_left (30) ; * d = d . wrapping_add (w [t + 1] . wrapping_add (e . rotate_left (5)) . wrapping_add (f1 (* a , * b , * c)) . wrapping_add (K [0]) ,) ; * a = a . rotate_left (30) ; * c = c . wrapping_add (w [t + 2] . wrapping_add (d . rotate_left (5)) . wrapping_add (f1 (* e , * a , * b)) . wrapping_add (K [0]) ,) ; * e = e . rotate_left (30) ; * b = b . wrapping_add (w [t + 3] . wrapping_add (c . rotate_left (5)) . wrapping_add (f1 (* d , * e , * a)) . wrapping_add (K [0]) ,) ; * d = d . rotate_left (30) ; }
    };
}

round1_step4!()