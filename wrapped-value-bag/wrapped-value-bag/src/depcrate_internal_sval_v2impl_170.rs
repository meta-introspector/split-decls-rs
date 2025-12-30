// Generated macro for impl_170 (impl)
macro_rules! Depcrate_internal_sval_v2impl_170 {
() => {
// Module: crate::internal::sval::v2
// Provides: {"impl_170"}
// Dependencies: {}
impl < 's , 'f > Slot < 's , 'f > { # [doc = " Fill the slot with a structured value."] # [doc = ""] # [doc = " The given value doesn't need to satisfy any particular lifetime constraints."] pub fn fill_sval2 < T > (self , value : T) -> Result < () , Error > where T : value_bag_sval2 :: lib :: Value , { self . fill (| visitor | visitor . sval2 (& value)) } }
};
}
