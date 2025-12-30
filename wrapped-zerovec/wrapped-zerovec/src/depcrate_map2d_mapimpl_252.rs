// Generated macro for impl_252 (impl)
macro_rules! Depcrate_map2d_mapimpl_252 {
() => {
// Module: crate::map2d::map
// Provides: {"impl_252"}
// Dependencies: {}
impl < 'a , A , B , C , K0 , K1 , V > FromIterator < (A , B , C) > for ZeroMap2d < 'a , K0 , K1 , V > where A : Borrow < K0 > , B : Borrow < K1 > , C : Borrow < V > , K0 : ZeroMapKV < 'a > + ? Sized + Ord , K1 : ZeroMapKV < 'a > + ? Sized + Ord , V : ZeroMapKV < 'a > + ? Sized , { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = (A , B , C) > , { let iter = iter . into_iter () ; let mut map = match iter . size_hint () { (_ , Some (upper)) => Self :: with_capacity (upper) , (lower , None) => Self :: with_capacity (lower) , } ; for (key0 , key1 , value) in iter { if let Some ((key0 , key1 , value)) = map . try_append (key0 . borrow () , key1 . borrow () , value . borrow ()) { map . insert (key0 , key1 , value) ; } } # [cfg (debug_assertions)] map . check_invariants () ; map } }
};
}
