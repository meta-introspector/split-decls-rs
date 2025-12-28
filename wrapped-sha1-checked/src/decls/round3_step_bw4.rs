macro_rules! round3_step_bw4 {
    () => {
        # [inline] fn round3_step_bw4 (a : & mut u32 , b : & mut u32 , c : & mut u32 , d : & mut u32 , e : & mut u32 , m : & [u32 ; 80] , t : usize ,) { round3_step_bw (* a , b , * c , * d , e , m [t]) ; round3_step_bw (* b , c , * d , * e , a , m [t - 1]) ; round3_step_bw (* c , d , * e , * a , b , m [t - 2]) ; round3_step_bw (* d , e , * a , * b , c , m [t - 3]) ; }
    };
}

round3_step_bw4!()