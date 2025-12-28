macro_rules! assert_align_ge {
    () => {
        # [doc = " Asserts that the types' alignments are greater than or equal to each other."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A pointer has greater alignment than `u8` and `i8`:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_ge!(*const u8, u8, i8);"] # [doc = " ```"] # [doc = ""] # [doc = " The following example fails to compile because a `u8` has smaller alignment"] # [doc = " than `usize`:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_align_ge!(u8, usize);"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! assert_align_ge { ($ x : ty , $ ($ y : ty) ,+ $ (,) ?) => { const _ : fn () = || { use $ crate :: _core :: mem :: align_of ; const_assert_ge ! (align_of ::<$ x > () $ (, align_of ::<$ y > ()) +) ; } ; } ; }
    };
}

assert_align_ge!()