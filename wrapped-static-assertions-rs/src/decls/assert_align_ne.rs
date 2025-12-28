macro_rules! assert_align_ne {
    () => {
        # [doc = " Asserts that the types' alignments are **not** equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A `u8` does not have the same alignment as a pointer:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_ne!(u8, *const u8);"] # [doc = " ```"] # [doc = ""] # [doc = " The following example fails to compile because a `usize` has the same"] # [doc = " alignment as a pointer:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_ne!(*const u8, usize);"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! assert_align_ne { ($ x : ty , $ ($ y : ty) ,+ $ (,) ?) => { const _ : fn () = || { use $ crate :: _core :: mem :: align_of ; const_assert_ne ! (align_of ::<$ x > () $ (, align_of ::<$ y > ()) +) ; } ; } ; }
    };
}

assert_align_ne!();