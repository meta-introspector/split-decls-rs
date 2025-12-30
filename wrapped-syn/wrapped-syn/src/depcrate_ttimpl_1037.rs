// Generated macro for impl_1037 (impl)
macro_rules! Depcrate_ttimpl_1037 {
() => {
// Module: crate::tt
// Provides: {"impl_1037"}
// Dependencies: {}
impl < 'a > PartialEq for TokenStreamHelper < 'a > { fn eq (& self , other : & Self) -> bool { let left = self . 0 . clone () . into_iter () ; let mut right = other . 0 . clone () . into_iter () ; for item1 in left { let item2 = match right . next () { Some (item) => item , None => return false , } ; if TokenTreeHelper (& item1) != TokenTreeHelper (& item2) { return false ; } } right . next () . is_none () } }
};
}
