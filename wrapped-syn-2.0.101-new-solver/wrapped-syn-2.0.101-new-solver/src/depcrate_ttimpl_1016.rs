// Generated macro for impl_1016 (impl)
macro_rules! Depcrate_ttimpl_1016 {
() => {
// Module: crate::tt
// Provides: {"impl_1016"}
// Dependencies: {}
impl < 'a > PartialEq for TokenTreeHelper < 'a > { fn eq (& self , other : & Self) -> bool { use proc_macro2 :: Spacing ; match (self . 0 , other . 0) { (TokenTree :: Group (g1) , TokenTree :: Group (g2)) => { match (g1 . delimiter () , g2 . delimiter ()) { (Delimiter :: Parenthesis , Delimiter :: Parenthesis) | (Delimiter :: Brace , Delimiter :: Brace) | (Delimiter :: Bracket , Delimiter :: Bracket) | (Delimiter :: None , Delimiter :: None) => { } _ => return false , } let s1 = g1 . stream () . into_iter () ; let mut s2 = g2 . stream () . into_iter () ; for item1 in s1 { let item2 = match s2 . next () { Some (item) => item , None => return false , } ; if TokenTreeHelper (& item1) != TokenTreeHelper (& item2) { return false ; } } s2 . next () . is_none () } (TokenTree :: Punct (o1) , TokenTree :: Punct (o2)) => { o1 . as_char () == o2 . as_char () && match (o1 . spacing () , o2 . spacing ()) { (Spacing :: Alone , Spacing :: Alone) | (Spacing :: Joint , Spacing :: Joint) => true , _ => false , } } (TokenTree :: Literal (l1) , TokenTree :: Literal (l2)) => l1 . to_string () == l2 . to_string () , (TokenTree :: Ident (s1) , TokenTree :: Ident (s2)) => s1 == s2 , _ => false , } } }
};
}
