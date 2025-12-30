// Generated macro for impl_166 (impl)
macro_rules! Depcrateimpl_166 {
() => {
// Module: crate
// Provides: {"impl_166"}
// Dependencies: {}
impl FutureIncompatibilityReason { pub fn edition (self) -> Option < Edition > { match self { Self :: EditionError (e) | Self :: EditionSemanticsChange (e) | Self :: EditionAndFutureReleaseError (e) | Self :: EditionAndFutureReleaseSemanticsChange (e) => Some (e) , FutureIncompatibilityReason :: FutureReleaseError | FutureIncompatibilityReason :: FutureReleaseSemanticsChange | FutureIncompatibilityReason :: Custom (_) => None , } } }
};
}
