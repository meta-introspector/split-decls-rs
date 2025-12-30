// Generated macro for impl_38 (impl)
macro_rules! Depcrate_attr_custom_metaimpl_38 {
() => {
// Module: crate::attr::custom_meta
// Provides: {"impl_38"}
// Dependencies: {}
impl Parse for CustomMeta { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { Ok (if input . peek (Token ! [!]) { Self :: Not (input . parse () ? , input . parse () ?) } else { Meta :: parse (input) ? . into () }) } }
};
}
