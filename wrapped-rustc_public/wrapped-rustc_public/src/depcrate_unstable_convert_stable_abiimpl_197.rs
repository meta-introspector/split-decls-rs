// Generated macro for impl_197 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_197 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: TagEncoding < rustc_abi :: VariantIdx > { type T = TagEncoding ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { rustc_abi :: TagEncoding :: Direct => TagEncoding :: Direct , rustc_abi :: TagEncoding :: Niche { untagged_variant , niche_variants , niche_start } => { TagEncoding :: Niche { untagged_variant : untagged_variant . stable (tables , cx) , niche_variants : niche_variants . stable (tables , cx) , niche_start : * niche_start , } } } } }
};
}
