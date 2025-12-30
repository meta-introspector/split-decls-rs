// Generated macro for OutputsRepackager (trait)
macro_rules! Depcrate_dist_pkgOutputsRepackager {
() => {
// Module: crate::dist::pkg
// Provides: {"OutputsRepackager"}
// Dependencies: {}
pub trait OutputsRepackager { fn repackage_outputs (self : Box < Self > , wtr : & mut dyn io :: Write) -> Result < dist :: PathTransformer > ; }
};
}
