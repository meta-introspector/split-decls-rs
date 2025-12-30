// Generated macro for impl_241 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_241 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_241"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: UserTypeProjection { type T = crate :: mir :: UserTypeProjection ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { UserTypeProjection { base : self . base . as_usize () , projection : opaque (& self . projs) } } }
};
}
