macro_rules! assert_align_gt {
    () => {
        # [doc = " Asserts that the types' alignments are greater than each other."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A pointer has greater alignment than `u16`, which has greater alignment than"] # [doc = " `u8`:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_gt!(*const u8, u16, u8);"] # [doc = " ```"] # [doc = ""] # [doc = " The following example fails to compile because a `usize` has the same"] # [doc = " alignment as a pointer:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_gt!(*const u8, usize);"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! assert_align_gt { ($ x : ty , $ ($ y : ty) ,+ $ (,) ?) => { const _ : fn () = || { use $ crate :: _core :: mem :: align_of ; const_assert_gt ! (align_of ::<$ x > () $ (, align_of ::<$ y > ()) +) ; } ; } ; }
    };
}

assert_align_gt!()