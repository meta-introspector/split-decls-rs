// Generated macro for OrphanCheckMode (enum)
macro_rules! Depcrate_coherenceOrphanCheckMode {
() => {
// Module: crate::coherence
// Provides: {"OrphanCheckMode"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub enum OrphanCheckMode { # [doc = " Proper orphan check."] Proper , # [doc = " Improper orphan check for backward compatibility."] # [doc = ""] # [doc = " In this mode, type params inside projections are considered to be covered"] # [doc = " even if the projection may normalize to a type that doesn't actually cover"] # [doc = " them. This is unsound. See also [#124559] and [#99554]."] # [doc = ""] # [doc = " [#124559]: https://github.com/rust-lang/rust/issues/124559"] # [doc = " [#99554]: https://github.com/rust-lang/rust/issues/99554"] Compat , }
};
}
