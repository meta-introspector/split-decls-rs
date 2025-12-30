// Generated macro for impl_348 (impl)
macro_rules! Depcrate_internedimpl_348 {
() => {
// Module: crate::interned
// Provides: {"impl_348"}
// Dependencies: {}
impl < A , T : Hash + Eq + PartialEq < A > > HashEqLike < & [A] > for Vec < T > { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , h) ; } fn eq (& self , data : & & [A]) -> bool { self . len () == data . len () && data . iter () . enumerate () . all (| (i , a) | & self [i] == a) } }
};
}
