// Generated macro for impl_47 (impl)
macro_rules! Depcrate_parserimpl_47 {
() => {
// Module: crate::parser
// Provides: {"impl_47"}
// Dependencies: {}
impl Event { # [doc = " Create an empty scalar."] fn empty_scalar () -> Event { Event :: Scalar (String :: new () , TScalarStyle :: Plain , 0 , None) } # [doc = " Create an empty scalar with the given anchor."] fn empty_scalar_with_anchor (anchor : usize , tag : Option < Tag >) -> Event { Event :: Scalar (String :: new () , TScalarStyle :: Plain , anchor , tag) } }
};
}
