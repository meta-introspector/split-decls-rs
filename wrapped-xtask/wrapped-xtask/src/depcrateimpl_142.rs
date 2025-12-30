// Generated macro for impl_142 (impl)
macro_rules! Depcrateimpl_142 {
() => {
// Module: crate
// Provides: {"impl_142"}
// Dependencies: {}
impl Cli { fn run (self) -> Result < () > { match self { Self :: Build (build) => build . run () , Self :: Ci (ci) => ci . run () , Self :: Clippy (clippy) => clippy . run () , Self :: Doc (doc) => doc . run () , } } }
};
}
