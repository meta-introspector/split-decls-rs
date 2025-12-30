// Generated macro for impl_basic_dim (macro)
macro_rules! Depcrate_complex_lstm_matriximpl_basic_dim {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_basic_dim"}
// Dependencies: {}
macro_rules ! impl_basic_dim { ($ t1 : path , $ t2 : path , $ t3 : path) => { impl <'a > $ t1 { # [allow (dead_code)] pub (super) fn dim (& self) -> usize { let [dim] = self . dims ; dim } } impl <'a > $ t2 { # [allow (dead_code)] pub (super) fn dim (& self) -> (usize , usize) { let [d0 , d1] = self . dims ; (d0 , d1) } } impl <'a > $ t3 { # [allow (dead_code)] pub (super) fn dim (& self) -> (usize , usize , usize) { let [d0 , d1 , d2] = self . dims ; (d0 , d1 , d2) } } } ; }
};
}
