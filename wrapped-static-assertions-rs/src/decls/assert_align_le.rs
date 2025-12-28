macro_rules! assert_align_le {
    () => {
        # [doc = " Asserts that the types' alignments are less than or equal to each other."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A `u8` and `i8` have smaller alignment than any pointer type:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_le!(u8, i8, *const u8);"] # [doc = " ```"] # [doc = ""] # [doc = " The following example fails to compile because a `usize` has greater"] # [doc = " alignment than `u8`:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_le!(usize, u8);"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! assert_align_le { ($ x : ty , $ ($ y : ty) ,+ $ (,) ?) => { const _ : fn () = || { use $ crate :: _core :: mem :: align_of ; const_assert_le ! (align_of ::<$ x > () $ (, align_of ::<$ y > ()) +) ; } ; } ; }
    };
}

assert_align_le!();