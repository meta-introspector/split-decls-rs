// Generated macro for impl_136 (impl)
macro_rules! Depcrate_headerimpl_136 {
() => {
// Module: crate::header
// Provides: {"impl_136"}
// Dependencies: {}
impl < H : PartialOrd , T : ? Sized + PartialOrd > PartialOrd for HeaderSlice < HeaderWithLength < H > , T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { (& self . header . header , & self . slice) . partial_cmp (& (& other . header . header , & other . slice)) } }
};
}
