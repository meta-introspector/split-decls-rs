// Generated macro for impl_1260 (impl)
macro_rules! Depcrate_ioimpl_1260 {
() => {
// Module: crate::io
// Provides: {"impl_1260"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < R : Read > Iterator for Bytes < R > { type Item = Result < u8 > ; fn next (& mut self) -> Option < Result < u8 > > { SpecReadByte :: spec_read_byte (& mut self . inner) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { SizeHint :: size_hint (& self . inner) } }
};
}
