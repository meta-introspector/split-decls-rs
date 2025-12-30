// Generated macro for impl_192 (impl)
macro_rules! Depcrate_macimpl_192 {
() => {
// Module: crate::mac
// Provides: {"impl_192"}
// Dependencies: {}
# [cfg (feature = "extra-traits")] impl PartialEq for TokenTree { fn eq (& self , other : & TokenTree) -> bool { use proc_macro2 :: Spacing ; match (& self . 0 . kind , & other . 0 . kind) { (& TokenNode :: Group (d1 , ref s1) , & TokenNode :: Group (d2 , ref s2)) => { match (d1 , d2) { (Delimiter :: Parenthesis , Delimiter :: Parenthesis) | (Delimiter :: Brace , Delimiter :: Brace) | (Delimiter :: Bracket , Delimiter :: Bracket) => { } (Delimiter :: None , Delimiter :: None) => { } _ => return false , } let s1 = s1 . clone () . into_iter () ; let mut s2 = s2 . clone () . into_iter () ; for item1 in s1 { let item2 = match s2 . next () { Some (item) => item , None => return false , } ; if TokenTree (item1) != TokenTree (item2) { return false } } s2 . next () . is_none () } (& TokenNode :: Op (o1 , k1) , & TokenNode :: Op (o2 , k2)) => { o1 == o2 && match (k1 , k2) { (Spacing :: Alone , Spacing :: Alone) | (Spacing :: Joint , Spacing :: Joint) => true , _ => false , } } (& TokenNode :: Literal (ref l1) , & TokenNode :: Literal (ref l2)) => { l1 . to_string () == l2 . to_string () } (& TokenNode :: Term (ref s1) , & TokenNode :: Term (ref s2)) => { s1 . as_str () == s2 . as_str () } _ => false , } } }
};
}
