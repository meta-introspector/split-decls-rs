// Generated macro for VariantMetadata (trait)
macro_rules! DepcrateVariantMetadata {
() => {
// Module: crate
// Provides: {"VariantMetadata"}
// Dependencies: {}
pub trait VariantMetadata { const VARIANT_COUNT : usize ; const VARIANT_NAMES : & 'static [& 'static str] ; fn variant_name (& self) -> & 'static str ; }
};
}
