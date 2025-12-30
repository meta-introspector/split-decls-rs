// Generated macro for impl_1019 (impl)
macro_rules! Depcrate_ttimpl_1019 {
() => {
// Module: crate::tt
// Provides: {"impl_1019"}
// Dependencies: {}
impl < 'a > PartialEq for TokenStreamHelper < 'a > { fn eq (& self , other : & Self) -> bool { let left = self . 0 . clone () . into_iter () . collect :: < Vec < _ > > () ; let right = other . 0 . clone () . into_iter () . collect :: < Vec < _ > > () ; if left . len () != right . len () { return false ; } for (a , b) in left . into_iter () . zip (right) { if TokenTreeHelper (& a) != TokenTreeHelper (& b) { return false ; } } true } }
};
}
