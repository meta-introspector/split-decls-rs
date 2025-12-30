// Generated macro for constructor (macro)
macro_rules! Depcrate_macrosconstructor {
() => {
// Module: crate::macros
// Provides: {"constructor"}
// Dependencies: {}
macro_rules ! constructor { ([$ ($ q : tt) *] $ r : ty , $ R : ty , $ p : ident , $ (# [$ pattr : meta]) * $ prop : ident => $ V : path $ ([$ ($ a : ident : $ T : ty) ,+]) ?) => { # [doc = " Returns `self` with the"] # [doc = concat ! ("[`" , stringify ! ($ p) , "()`](Self::" , stringify ! ($ p) , "())")] # [doc = " set to"] # [doc = concat ! ("[`" , stringify ! ($ V) , "`].")] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = concat ! ("# let value = yansi::Painted::new(0);" , $ ($ ("\n# let " , stringify ! ($ a) , " = 0;") ,+) ?)] # [doc = concat ! ("println!(\"{}\", value." , stringify ! ($ prop) , "(" , $ (stringify ! ($ ($ a) ,+) ,) ? "));")] # [doc = " ```"] # [inline] $ (# [$ pattr]) * $ ($ q) * fn $ prop (self : $ r $ ($ (,$ a : $ T) +) ?) -> $ R { let v = $ V $ (($ ($ a) ,*)) ?; self . apply (crate :: style :: Application ::$ p (v)) } } ; ([$ ($ q : tt) *] $ (# [$ attr : meta]) * $ r : ty , $ R : ty , $ kind : ident ($ A : ty)) => { $ (# [$ attr]) * # [inline] $ ($ q) * fn $ kind (self : $ r , value : $ A) -> $ R { self . apply (crate :: style :: Application ::$ kind (value)) } } ; }
};
}
