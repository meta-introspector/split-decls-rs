// Generated macro for impl_338 (impl)
macro_rules! Depcrate_internedimpl_338 {
() => {
// Module: crate::interned
// Provides: {"impl_338"}
// Dependencies: {}
impl < T > HashEqLike < & T > for T where T : Hash + Eq , { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , & mut * h) ; } fn eq (& self , data : & & T) -> bool { * self == * * data } }
};
}
