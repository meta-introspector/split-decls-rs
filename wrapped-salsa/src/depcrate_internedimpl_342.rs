// Generated macro for impl_342 (impl)
macro_rules! Depcrate_internedimpl_342 {
() => {
// Module: crate::interned
// Provides: {"impl_342"}
// Dependencies: {}
impl < 'a , T > HashEqLike < & 'a T > for Arc < T > where T : ? Sized + Hash + Eq , Arc < T > : From < & 'a T > , { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (& * * self , & mut * h) } fn eq (& self , data : & & T) -> bool { * * self == * * data } }
};
}
