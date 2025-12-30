// Generated macro for impl_252 (impl)
macro_rules! Depcrate_add_extensionimpl_252 {
() => {
// Module: crate::add_extension
// Provides: {"impl_252"}
// Dependencies: {}
impl < S , T > Layer < S > for AddExtensionLayer < T > where T : Clone , { type Service = AddExtension < S , T > ; fn layer (& self , inner : S) -> Self :: Service { AddExtension { inner , value : self . value . clone () , } } }
};
}
