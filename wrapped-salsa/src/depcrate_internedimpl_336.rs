// Generated macro for impl_336 (impl)
macro_rules! Depcrate_internedimpl_336 {
() => {
// Module: crate::interned
// Provides: {"impl_336"}
// Dependencies: {}
impl < T > HashEqLike < T > for T where T : Hash + Eq , { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , & mut * h) ; } fn eq (& self , data : & T) -> bool { self == data } }
};
}
