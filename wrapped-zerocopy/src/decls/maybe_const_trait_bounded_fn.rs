macro_rules! maybe_const_trait_bounded_fn {
    () => {
        # [doc = " Emits a function definition as either `const fn` or `fn` depending on"] # [doc = " whether the current toolchain version supports `const fn` with generic trait"] # [doc = " bounds."] macro_rules ! maybe_const_trait_bounded_fn { ($ (# [$ attr : meta]) * $ vis : vis const fn $ name : ident ($ ($ args : ident $ (: $ arg_tys : ty) ?) ,* $ (,) ?) $ (-> $ ret_ty : ty) ? $ body : block) => { # [cfg (zerocopy_generic_bounds_in_const_fn_1_61_0)] $ (# [$ attr]) * $ vis const fn $ name ($ ($ args $ (: $ arg_tys) ?) ,*) $ (-> $ ret_ty) ? $ body # [cfg (not (zerocopy_generic_bounds_in_const_fn_1_61_0))] $ (# [$ attr]) * $ vis fn $ name ($ ($ args $ (: $ arg_tys) ?) ,*) $ (-> $ ret_ty) ? $ body } ; }
    };
}

maybe_const_trait_bounded_fn!();