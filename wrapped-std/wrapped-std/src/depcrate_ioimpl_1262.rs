// Generated macro for impl_1262 (impl)
macro_rules! Depcrate_ioimpl_1262 {
() => {
// Module: crate::io
// Provides: {"impl_1262"}
// Dependencies: {}
impl < R > SpecReadByte for R where Self : Read , { # [inline] default fn spec_read_byte (& mut self) -> Option < Result < u8 > > { inlined_slow_read_byte (self) } }
};
}
