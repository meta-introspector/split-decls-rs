// Generated macro for impl_828 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_828 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_828"}
// Dependencies: {}
impl Hash for PrimitiveDateTime { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_i128 () . hash (state) ; } }
};
}
