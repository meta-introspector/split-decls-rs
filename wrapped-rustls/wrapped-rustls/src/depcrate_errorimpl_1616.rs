// Generated macro for impl_1616 (impl)
macro_rules! Depcrate_errorimpl_1616 {
() => {
// Module: crate::error
// Provides: {"impl_1616"}
// Dependencies: {}
impl ExtendedKeyPurpose { pub (crate) fn for_values (values : impl Iterator < Item = usize >) -> Self { let values = values . collect :: < Vec < _ > > () ; match & * values { ExtendedKeyUsage :: CLIENT_AUTH_REPR => Self :: ClientAuth , ExtendedKeyUsage :: SERVER_AUTH_REPR => Self :: ServerAuth , _ => Self :: Other (values) , } } }
};
}
