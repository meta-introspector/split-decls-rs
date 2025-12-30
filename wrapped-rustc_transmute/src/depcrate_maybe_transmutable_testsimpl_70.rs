// Generated macro for impl_70 (impl)
macro_rules! Depcrate_maybe_transmutable_testsimpl_70 {
() => {
// Module: crate::maybe_transmutable::tests
// Provides: {"impl_70"}
// Dependencies: {}
impl Representation for Dfa { fn is_transmutable (src : Self , dst : Self , assume : Assume) -> Answer < ! , ! > { crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (src , dst , assume , UltraMinimal :: default () ,) . answer () } }
};
}
