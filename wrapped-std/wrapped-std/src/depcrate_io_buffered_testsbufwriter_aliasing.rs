// Generated macro for bufwriter_aliasing (function)
macro_rules! Depcrate_io_buffered_testsbufwriter_aliasing {
() => {
// Module: crate::io::buffered::tests
// Provides: {"bufwriter_aliasing"}
// Dependencies: {}
# [doc = " This is a regression test for https://github.com/rust-lang/rust/issues/127584."] # [test] fn bufwriter_aliasing () { use crate :: io :: { BufWriter , Cursor } ; let mut v = vec ! [0 ; 1024] ; let c = Cursor :: new (& mut v) ; let w = BufWriter :: new (Box :: new (c)) ; let _ = w . into_parts () ; }
};
}
