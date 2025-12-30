// Generated macro for impl_340 (impl)
macro_rules! Depcrate_internedimpl_340 {
() => {
// Module: crate::interned
// Provides: {"impl_340"}
// Dependencies: {}
impl < 'a , T > HashEqLike < & 'a T > for Box < T > where T : ? Sized + Hash + Eq , Box < T > : From < & 'a T > , { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , & mut * h) } fn eq (& self , data : & & T) -> bool { * * self == * * data } }
};
}
