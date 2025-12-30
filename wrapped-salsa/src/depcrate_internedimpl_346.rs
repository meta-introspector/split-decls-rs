// Generated macro for impl_346 (impl)
macro_rules! Depcrate_internedimpl_346 {
() => {
// Module: crate::interned
// Provides: {"impl_346"}
// Dependencies: {}
impl HashEqLike < & str > for String { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , & mut * h) } fn eq (& self , data : & & str) -> bool { self == * data } }
};
}
