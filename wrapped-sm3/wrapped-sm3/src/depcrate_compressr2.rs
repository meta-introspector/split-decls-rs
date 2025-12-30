// Generated macro for R2 (macro)
macro_rules! Depcrate_compressR2 {
() => {
// Module: crate::compress
// Provides: {"R2"}
// Dependencies: {}
macro_rules ! R2 { ($ a : ident , $ b : ident , $ c : ident , $ d : ident , $ e : ident , $ f : ident , $ g : ident , $ h : ident , $ t : expr , $ w1 : expr , $ w2 : expr) => { { let out = sm3_round2 ($ a , $ b , $ c , $ d , $ e , $ f , $ g , $ h , $ t , $ w1 , $ w2) ; $ a = out [0] ; $ b = out [1] ; $ c = out [2] ; $ d = out [3] ; $ e = out [4] ; $ f = out [5] ; $ g = out [6] ; $ h = out [7] ; } } ; }
};
}
