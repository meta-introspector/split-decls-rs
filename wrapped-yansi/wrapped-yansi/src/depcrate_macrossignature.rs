// Generated macro for signature (macro)
macro_rules! Depcrate_macrossignature {
() => {
// Module: crate::macros
// Provides: {"signature"}
// Dependencies: {}
macro_rules ! signature { ([$ ($ q : tt) *] $ r : ty , $ R : ty , $ p : ident , $ (# [$ pattr : meta]) * $ prop : ident => $ V : path $ ([$ ($ a : ident : $ T : ty) ,+]) ?) => { # [doc = " Returns `self` with the"] # [doc = concat ! ("[`" , stringify ! ($ p) , "()`](Self::" , stringify ! ($ p) , "())")] # [doc = " set to"] # [doc = concat ! ("[`" , stringify ! ($ V) , "`].")] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = concat ! ("# let value = yansi::Painted::new(0);" , $ ($ ("\n# let " , stringify ! ($ a) , " = 0;") ,+) ?)] # [doc = concat ! ("println!(\"{}\", value." , stringify ! ($ prop) , "(" , $ (stringify ! ($ ($ a) ,+) ,) ? "));")] # [doc = " ```"] $ (# [$ pattr]) * $ ($ q) * fn $ prop (self : $ r $ ($ (,$ a : $ T) +) ?) -> $ R ; } ; ([$ ($ q : tt) *] $ (# [$ attr : meta]) * $ r : ty , $ R : ty , $ kind : ident ($ A : ty)) => { $ (# [$ attr]) * $ ($ q) * fn $ kind (self : $ r , value : $ A) -> $ R ; } ; }
};
}
