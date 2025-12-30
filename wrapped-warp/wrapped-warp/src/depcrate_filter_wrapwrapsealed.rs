// Generated macro for WrapSealed (trait)
macro_rules! Depcrate_filter_wrapWrapSealed {
() => {
// Module: crate::filter::wrap
// Provides: {"WrapSealed"}
// Dependencies: {}
pub trait WrapSealed < F : Filter > { type Wrapped : Filter ; fn wrap (& self , filter : F) -> Self :: Wrapped ; }
};
}
