macro_rules! const_panic {
    () => {
        # [doc = " Either panic (if the current Rust toolchain supports panicking in `const"] # [doc = " fn`) or evaluate a constant that will cause an array indexing error whose"] # [doc = " error message will include the format string."] # [doc = ""] # [doc = " The type that this expression evaluates to must be `Copy`, or else the"] # [doc = " non-panicking desugaring will fail to compile."] macro_rules ! const_panic { (@ non_panic $ ($ _arg : tt) +) => { { let panic : [_ ; 0] = [] ; # [allow (unconditional_panic)] panic [0] } } ; ($ ($ arg : tt) +) => { { # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] panic ! ($ ($ arg) +) ; # [cfg (not (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0))] const_panic ! (@ non_panic $ ($ arg) +) } } ; }
    };
}

const_panic!();