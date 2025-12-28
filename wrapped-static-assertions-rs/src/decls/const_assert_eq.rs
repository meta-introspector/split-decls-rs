macro_rules! const_assert_eq {
    () => {
        # [doc = " Asserts that constants are equal in value."] # [doc = ""] # [doc = " Use [`const_assert_eq_usize!`](macro.const_assert_eq_usize.html) for better"] # [doc = " error messages when asserting"] # [doc = " [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) equality."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " This works as a shorthand for `const_assert!(a == b)`:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " const TWO: i32 = 2;"] # [doc = ""] # [doc = " const_assert_eq!(TWO * TWO, TWO + TWO);"] # [doc = " ```"] # [doc = ""] # [doc = " Just because 2 × 2 = 2 + 2 doesn't mean it holds true for other numbers:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " const_assert_eq!(4 + 4, 4 * 4);"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! const_assert_eq { ($ x : expr , $ ($ y : expr) ,+ $ (,) ?) => { const_assert ! ($ ($ x == $ y) &&+) ; } ; }
    };
}

const_assert_eq!();