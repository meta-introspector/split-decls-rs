// Generated macro for FieldInfo (struct)
macro_rules! Depcrate_diagnostics_utilsFieldInfo {
() => {
// Module: crate::diagnostics::utils
// Provides: {"FieldInfo"}
// Dependencies: {}
# [doc = " Field information passed to the builder. Deliberately omits attrs to discourage the"] # [doc = " `generate_*` methods from walking the attributes themselves."] pub (crate) struct FieldInfo < 'a > { pub (crate) binding : & 'a BindingInfo < 'a > , pub (crate) ty : FieldInnerTy < 'a > , pub (crate) span : & 'a proc_macro2 :: Span , }
};
}
