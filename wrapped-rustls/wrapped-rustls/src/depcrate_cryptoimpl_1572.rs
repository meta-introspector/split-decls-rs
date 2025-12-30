// Generated macro for impl_1572 (impl)
macro_rules! Depcrate_cryptoimpl_1572 {
() => {
// Module: crate::crypto
// Provides: {"impl_1572"}
// Dependencies: {}
impl StartedKeyExchange { # [doc = " Collapses this object into its underlying [`ActiveKeyExchange`]."] # [doc = ""] # [doc = " This removes the ability to do the hybrid key exchange optimization,"] # [doc = " but still allows the key exchange as a whole to be completed."] pub fn into_single (self) -> Box < dyn ActiveKeyExchange > { match self { Self :: Single (s) => s , Self :: Hybrid (h) => h . into_key_exchange () , } } # [doc = " Accesses the [`HybridKeyExchange`], and checks it was also usable separately."] # [doc = ""] # [doc = " Returns:"] # [doc = ""] # [doc = " - the [`HybridKeyExchange`]"] # [doc = " - the stand-alone `SupportedKxGroup` for the hybrid's component group."] # [doc = ""] # [doc = " This returns `None` for:"] # [doc = ""] # [doc = " - non-hybrid groups,"] # [doc = " - if the hybrid component group is not present in `supported`"] # [doc = " - if the hybrid component group is not usable with `version`"] pub (crate) fn as_hybrid_checked (& self , supported : & [& 'static dyn SupportedKxGroup] , version : ProtocolVersion ,) -> Option < (& dyn HybridKeyExchange , & 'static dyn SupportedKxGroup) > { let Self :: Hybrid (hybrid) = self else { return None ; } ; let component_group = hybrid . component () . 0 ; if ! component_group . usable_for_version (version) { return None ; } supported . iter () . find (| g | g . name () == component_group) . copied () . map (| g | (hybrid . as_ref () , g)) } }
};
}
