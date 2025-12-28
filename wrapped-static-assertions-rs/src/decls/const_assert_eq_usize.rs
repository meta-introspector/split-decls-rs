macro_rules! const_assert_eq_usize {
    () => {
        # [doc = " Asserts that constants of type"] # [doc = " [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) are equal in"] # [doc = " value."] # [doc = ""] # [doc = " This is equivalent to [`const_assert_eq!`](macro.const_assert_eq.html) but"] # [doc = " allows for inspecting the values in error messages."] # [macro_export] macro_rules ! const_assert_eq_usize { ($ x : expr , $ ($ y : expr) ,+ $ (,) ?) => { $ (const _ : [() ; $ x] = [() ; $ y] ;) + } ; }
    };
}

const_assert_eq_usize!()