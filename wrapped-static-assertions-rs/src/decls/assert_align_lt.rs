macro_rules! assert_align_lt {
    () => {
        # [doc = " Asserts that the types' alignments are less than each other."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A `u8` has smaller alignment than `u16`, which has smaller alignment than"] # [doc = " a pointer:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_lt!(u8, u16, *const u8);"] # [doc = " ```"] # [doc = ""] # [doc = " The following example fails to compile because a `usize` has the same"] # [doc = " alignment as a pointer:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_lt!(*const u8, usize);"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! assert_align_lt { ($ x : ty , $ ($ y : ty) ,+ $ (,) ?) => { const _ : fn () = || { use $ crate :: _core :: mem :: align_of ; const_assert_lt ! (align_of ::<$ x > () $ (, align_of ::<$ y > ()) +) ; } ; } ; }
    };
}

assert_align_lt!()