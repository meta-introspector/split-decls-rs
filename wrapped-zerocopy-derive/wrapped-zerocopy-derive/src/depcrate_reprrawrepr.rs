// Generated macro for RawRepr (enum)
macro_rules! Depcrate_reprRawRepr {
() => {
// Module: crate::repr
// Provides: {"RawRepr"}
// Dependencies: {}
# [doc = " The result of parsing a single `#[repr(...)]` attribute or a single"] # [doc = " directive inside a compound `#[repr(..., ...)]` attribute."] # [derive (Copy , Clone , PartialEq , Eq)] # [cfg_attr (test , derive (Debug))] pub (crate) enum RawRepr { Transparent , C , Rust , U8 , U16 , U32 , U64 , U128 , Usize , I8 , I16 , I32 , I64 , I128 , Isize , Align (NonZeroU32) , PackedN (NonZeroU32) , Packed , }
};
}
