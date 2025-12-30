// Generated macro for impl_25 (impl)
macro_rules! Depcrate_asciiimpl_25 {
() => {
// Module: crate::ascii
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < const N : usize > PartialEq < alloc :: string :: String > for TinyAsciiStr < N > { fn eq (& self , other : & alloc :: string :: String) -> bool { self . deref () == other . deref () } }
};
}
