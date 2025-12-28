macro_rules! const_assert {
    () => {
        # [doc = " Either assert (if the current Rust toolchain supports panicking in `const"] # [doc = " fn`) or evaluate the expression and, if it evaluates to `false`, call"] # [doc = " `const_panic!`. This is used in place of `assert!` in const contexts to"] # [doc = " accommodate old toolchains."] macro_rules ! const_assert { ($ e : expr) => { { # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] assert ! ($ e) ; # [cfg (not (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0))] { let e = $ e ; if ! e { let _ : () = const_panic ! (@ non_panic concat ! ("assertion failed: " , stringify ! ($ e))) ; } } } } ; ($ e : expr , $ ($ args : tt) +) => { { # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] assert ! ($ e , $ ($ args) +) ; # [cfg (not (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0))] { let e = $ e ; if ! e { let _ : () = const_panic ! (@ non_panic concat ! ("assertion failed: " , stringify ! ($ e) , ": " , stringify ! ($ arg)) , $ ($ args) *) ; } } } } ; }
    };
}

const_assert!();