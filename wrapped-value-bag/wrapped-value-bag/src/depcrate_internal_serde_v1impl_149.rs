// Generated macro for impl_149 (impl)
macro_rules! Depcrate_internal_serde_v1impl_149 {
() => {
// Module: crate::internal::serde::v1
// Provides: {"impl_149"}
// Dependencies: {}
impl < 's , 'f > Slot < 's , 'f > { # [doc = " Fill the slot with a structured value."] # [doc = ""] # [doc = " The given value doesn't need to satisfy any particular lifetime constraints."] pub fn fill_serde1 < T > (self , value : T) -> Result < () , Error > where T : value_bag_serde1 :: lib :: Serialize , { self . fill (| visitor | visitor . serde1 (& value)) } }
};
}
