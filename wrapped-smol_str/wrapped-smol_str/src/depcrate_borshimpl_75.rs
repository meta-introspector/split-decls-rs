// Generated macro for impl_75 (impl)
macro_rules! Depcrate_borshimpl_75 {
() => {
// Module: crate::borsh
// Provides: {"impl_75"}
// Dependencies: {}
impl BorshSerialize for SmolStr { fn serialize < W : Write > (& self , writer : & mut W) -> borsh :: io :: Result < () > { self . as_str () . serialize (writer) } }
};
}
