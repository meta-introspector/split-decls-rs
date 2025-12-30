// Generated macro for impl_79 (impl)
macro_rules! Depcrate_installimpl_79 {
() => {
// Module: crate::install
// Provides: {"impl_79"}
// Dependencies: {}
impl ServerOpt { fn to_features (& self) -> Vec < & 'static str > { let mut features = Vec :: new () ; features . extend (self . malloc . to_features ()) ; if self . force_always_assert { features . extend (["--features" , "force-always-assert"]) ; } features } }
};
}
