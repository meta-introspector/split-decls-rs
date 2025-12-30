// Generated macro for array_impl (macro)
macro_rules! Depcrate_hasharray_impl {
() => {
// Module: crate::hash
// Provides: {"array_impl"}
// Dependencies: {}
macro_rules ! array_impl { ($ s : expr , $ ($ size : expr) ,+) => { array_impl ! ($ s) ; $ (array_impl ! { $ size }) * } ; ($ size : expr) => { # [cfg (not (feature = "const-generics"))] impl Adler32Hash for [u8 ; $ size] { fn hash (& self) -> u32 { let mut hash = Adler32 :: new () ; hash . write (self) ; hash . finish () } } } ; }
};
}
