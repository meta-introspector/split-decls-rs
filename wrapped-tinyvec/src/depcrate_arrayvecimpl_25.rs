// Generated macro for impl_25 (impl)
macro_rules! Depcrate_arrayvecimpl_25 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (feature = "borsh")] # [cfg_attr (docs_rs , doc (cfg (feature = "borsh")))] impl < A : Array > borsh :: BorshSerialize for ArrayVec < A > where < A as Array > :: Item : borsh :: BorshSerialize , { fn serialize < W : borsh :: io :: Write > (& self , writer : & mut W ,) -> borsh :: io :: Result < () > { < usize as borsh :: BorshSerialize > :: serialize (& self . len () , writer) ? ; for elem in self . iter () { < < A as Array > :: Item as borsh :: BorshSerialize > :: serialize (elem , writer) ? ; } Ok (()) } }
};
}
