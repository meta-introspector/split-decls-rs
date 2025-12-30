// Generated macro for impl_350 (impl)
macro_rules! Depcrate_internedimpl_350 {
() => {
// Module: crate::interned
// Provides: {"impl_350"}
// Dependencies: {}
impl < const N : usize , A , T : Hash + Eq + PartialEq < A > > HashEqLike < [A ; N] > for Vec < T > { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , h) ; } fn eq (& self , data : & [A ; N]) -> bool { self . len () == data . len () && data . iter () . enumerate () . all (| (i , a) | & self [i] == a) } }
};
}
