// Generated macro for SetMember (trait)
macro_rules! Depcrate_setSetMember {
() => {
// Module: crate::set
// Provides: {"SetMember"}
// Dependencies: {}
pub trait SetMember : Copy + fmt :: Debug { const MAX_VALUE : u8 ; fn bit_mask (self) -> u16 ; fn from_bit_mask (value : u16) -> Option < Self > ; }
};
}
