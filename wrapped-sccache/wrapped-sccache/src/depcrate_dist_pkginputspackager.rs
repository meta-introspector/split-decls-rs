// Generated macro for InputsPackager (trait)
macro_rules! Depcrate_dist_pkgInputsPackager {
() => {
// Module: crate::dist::pkg
// Provides: {"InputsPackager"}
// Dependencies: {}
pub trait InputsPackager : Send { fn write_inputs (self : Box < Self > , wtr : & mut dyn io :: Write) -> Result < dist :: PathTransformer > ; }
};
}
