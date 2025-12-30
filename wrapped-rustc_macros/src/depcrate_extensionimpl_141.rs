// Generated macro for impl_141 (impl)
macro_rules! Depcrate_extensionimpl_141 {
() => {
// Module: crate::extension
// Provides: {"impl_141"}
// Dependencies: {}
impl Parse for ExtensionAttr { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let vis = input . parse () ? ; let _ : Token ! [trait] = input . parse () ? ; let trait_ = input . parse () ? ; Ok (ExtensionAttr { vis , trait_ }) } }
};
}
