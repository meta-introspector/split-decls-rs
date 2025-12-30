// Generated macro for impl_40 (impl)
macro_rules! Depcrate_forsimpl_40 {
() => {
// Module: crate::fors
// Provides: {"impl_40"}
// Dependencies: {}
impl < P : ForsParams > ForsMTSig < P > { const SIZE : usize = P :: N :: USIZE + P :: A :: USIZE * P :: N :: USIZE ; fn write_to (& self , slice : & mut [u8]) { debug_assert ! (slice . len () == Self :: SIZE , "Writing FORS MT sig to slice of incorrect length") ; slice . chunks_exact_mut (P :: N :: USIZE) . enumerate () . for_each (| (i , c) | { if i == 0 { c . copy_from_slice (& self . sk) ; } else { c . copy_from_slice (& self . auth [i - 1]) ; } }) ; } }
};
}
