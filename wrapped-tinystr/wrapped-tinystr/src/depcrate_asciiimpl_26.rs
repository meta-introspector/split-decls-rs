// Generated macro for impl_26 (impl)
macro_rules! Depcrate_asciiimpl_26 {
() => {
// Module: crate::ascii
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < const N : usize > PartialEq < TinyAsciiStr < N > > for alloc :: string :: String { fn eq (& self , other : & TinyAsciiStr < N >) -> bool { self . deref () == other . deref () } }
};
}
