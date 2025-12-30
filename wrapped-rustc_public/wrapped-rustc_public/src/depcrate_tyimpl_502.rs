// Generated macro for impl_502 (impl)
macro_rules! Depcrate_tyimpl_502 {
() => {
// Module: crate::ty
// Provides: {"impl_502"}
// Dependencies: {}
impl VariantDef { pub fn name (& self) -> Symbol { with (| cx | cx . variant_name (* self)) } # [doc = " Retrieve all the fields in this variant."] pub fn fields (& self) -> Vec < FieldDef > { with (| cx | cx . variant_fields (* self)) } }
};
}
