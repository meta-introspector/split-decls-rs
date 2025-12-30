// Generated macro for impl_347 (impl)
macro_rules! Depcrate_internedimpl_347 {
() => {
// Module: crate::interned
// Provides: {"impl_347"}
// Dependencies: {}
# [cfg (feature = "compact_str")] impl HashEqLike < & str > for compact_str :: CompactString { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , & mut * h) } fn eq (& self , data : & & str) -> bool { self == * data } }
};
}
