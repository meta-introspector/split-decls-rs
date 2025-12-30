// Generated macro for impl_142 (impl)
macro_rules! Depcrate_map_mapimpl_142 {
() => {
// Module: crate::map::map
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'a , A , B , K , V > FromIterator < (A , B) > for ZeroMap < 'a , K , V > where A : Borrow < K > , B : Borrow < V > , K : ZeroMapKV < 'a > + ? Sized + Ord , V : ZeroMapKV < 'a > + ? Sized , { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = (A , B) > , { let iter = iter . into_iter () ; let mut map = match iter . size_hint () { (_ , Some (upper)) => Self :: with_capacity (upper) , (lower , None) => Self :: with_capacity (lower) , } ; for (key , value) in iter { if let Some ((key , value)) = map . try_append (key . borrow () , value . borrow ()) { map . insert (key , value) ; } } map } }
};
}
