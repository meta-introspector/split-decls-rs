// Generated macro for impl_137 (impl)
macro_rules! Depcrate_headerimpl_137 {
() => {
// Module: crate::header
// Provides: {"impl_137"}
// Dependencies: {}
impl < H : Ord , T : ? Sized + Ord > Ord for HeaderSlice < HeaderWithLength < H > , T > { fn cmp (& self , other : & Self) -> Ordering { (& self . header . header , & self . slice) . cmp (& (& other . header . header , & other . slice)) } }
};
}
