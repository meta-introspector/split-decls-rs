// Generated macro for impl_441 (impl)
macro_rules! Depcrate_targetimpl_441 {
() => {
// Module: crate::target
// Provides: {"impl_441"}
// Dependencies: {}
impl MachineSize { # [inline (always)] pub fn bytes (self) -> usize { self . num_bits / 8 } # [inline (always)] pub fn bits (self) -> usize { self . num_bits } # [inline (always)] pub fn from_bits (num_bits : usize) -> MachineSize { MachineSize { num_bits } } # [inline] pub fn unsigned_int_max (self) -> Option < u128 > { (self . num_bits <= 128) . then (| | u128 :: MAX >> (128 - self . bits ())) } }
};
}
