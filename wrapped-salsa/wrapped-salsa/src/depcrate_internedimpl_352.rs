// Generated macro for impl_352 (impl)
macro_rules! Depcrate_internedimpl_352 {
() => {
// Module: crate::interned
// Provides: {"impl_352"}
// Dependencies: {}
impl HashEqLike < & Path > for PathBuf { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , h) ; } fn eq (& self , data : & & Path) -> bool { self == data } }
};
}
