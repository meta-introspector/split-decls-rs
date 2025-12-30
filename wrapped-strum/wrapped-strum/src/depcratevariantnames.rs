// Generated macro for VariantNames (trait)
macro_rules! DepcrateVariantNames {
() => {
// Module: crate
// Provides: {"VariantNames"}
// Dependencies: {}
# [doc = " A trait for retrieving the names of each variant in Enum. This trait can"] # [doc = " be autoderived by `strum_macros`."] pub trait VariantNames { # [doc = " Names of the variants of this enum"] const VARIANTS : & 'static [& 'static str] ; }
};
}
