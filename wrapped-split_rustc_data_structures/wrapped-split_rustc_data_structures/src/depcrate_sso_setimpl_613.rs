// Generated macro for impl_613 (impl)
macro_rules! Depcrate_sso_setimpl_613 {
() => {
// Module: crate::sso::set
// Provides: {"impl_613"}
// Dependencies: {}
impl < T : Eq + Hash > FromIterator < T > for SsoHashSet < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> SsoHashSet < T > { let mut set : SsoHashSet < T > = Default :: default () ; set . extend (iter) ; set } }
};
}
