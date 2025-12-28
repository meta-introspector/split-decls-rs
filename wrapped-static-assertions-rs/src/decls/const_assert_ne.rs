macro_rules! const_assert_ne {
    () => {
        # [doc = " Asserts that constants are **not** equal in value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " This works as a shorthand for `const_assert!(a != b)`:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " const NUM: usize = 32;"] # [doc = ""] # [doc = " const_assert_ne!(NUM * NUM, 64);"] # [doc = " ```"] # [doc = ""] # [doc = " The following example fails to compile because 2 is magic and 2 × 2 = 2 + 2:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " const_assert_ne!(2 + 2, 2 * 2);"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! const_assert_ne { ($ x : expr , $ ($ y : expr) ,+ $ (,) ?) => { const_assert ! ($ ($ x != $ y) &&+) ; } ; }
    };
}

const_assert_ne!()