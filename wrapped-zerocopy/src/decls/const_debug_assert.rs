macro_rules! const_debug_assert {
    () => {
        # [doc = " Like `const_assert!`, but relative to `debug_assert!`."] macro_rules ! const_debug_assert { ($ e : expr $ (, $ msg : expr) ?) => { { # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] debug_assert ! ($ e $ (, $ msg) ?) ; # [cfg (not (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0))] { if cfg ! (debug_assertions) { let e = $ e ; if ! e { let _ : () = const_panic ! (@ non_panic concat ! ("assertion failed: " , stringify ! ($ e) $ (, ": " , $ msg) ?)) ; } } } } } }
    };
}

const_debug_assert!();