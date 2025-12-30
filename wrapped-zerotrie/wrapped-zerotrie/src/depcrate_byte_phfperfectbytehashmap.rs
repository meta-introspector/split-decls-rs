// Generated macro for PerfectByteHashMap (struct)
macro_rules! Depcrate_byte_phfPerfectByteHashMap {
() => {
// Module: crate::byte_phf
// Provides: {"PerfectByteHashMap"}
// Dependencies: {}
# [doc = " A constant-time map from bytes to unique indices."] # [doc = ""] # [doc = " Uses a perfect hash function (see module-level documentation). Does not support mutation."] # [doc = ""] # [doc = " Standard layout: P, N bytes of Q, N bytes of expected keys"] # [derive (Debug , PartialEq , Eq)] # [repr (transparent)] pub struct PerfectByteHashMap < Store : ? Sized > (Store) ;
};
}
