// Generated macro for impl_46 (impl)
macro_rules! Depcrate_unicodeimpl_46 {
() => {
// Module: crate::unicode
// Provides: {"impl_46"}
// Dependencies: {}
impl < S1 : AsRef < str > , S2 : AsRef < str > > PartialEq < Unicode < S2 > > for Unicode < S1 > { # [inline] fn eq (& self , other : & Unicode < S2 >) -> bool { let mut left = self . 0 . as_ref () . chars () . flat_map (lookup) ; let mut right = other . 0 . as_ref () . chars () . flat_map (lookup) ; loop { let x = match left . next () { None => return right . next () . is_none () , Some (val) => val , } ; let y = match right . next () { None => return false , Some (val) => val , } ; if x != y { return false ; } } } }
};
}
