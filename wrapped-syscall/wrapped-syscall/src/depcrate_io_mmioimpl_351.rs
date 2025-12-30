// Generated macro for impl_351 (impl)
macro_rules! Depcrate_io_mmioimpl_351 {
() => {
// Module: crate::io::mmio
// Provides: {"impl_351"}
// Dependencies: {}
# [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] impl < T > Io for Mmio < T > where T : Copy + PartialEq + BitAnd < Output = T > + BitOr < Output = T > + Not < Output = T > , { type Value = T ; fn read (& self) -> T { unsafe { ptr :: read_volatile (ptr :: addr_of ! (self . value) . cast :: < T > ()) } } fn write (& mut self , value : T) { unsafe { ptr :: write_volatile (ptr :: addr_of_mut ! (self . value) . cast :: < T > () , value) } ; } }
};
}
