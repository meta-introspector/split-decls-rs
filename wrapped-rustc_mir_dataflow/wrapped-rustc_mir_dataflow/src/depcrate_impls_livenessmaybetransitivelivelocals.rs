// Generated macro for MaybeTransitiveLiveLocals (struct)
macro_rules! Depcrate_impls_livenessMaybeTransitiveLiveLocals {
() => {
// Module: crate::impls::liveness
// Provides: {"MaybeTransitiveLiveLocals"}
// Dependencies: {}
# [doc = " Like `MaybeLiveLocals`, but does not mark locals as live if they are used in a dead assignment."] # [doc = ""] # [doc = " This is basically written for dead store elimination and nothing else."] # [doc = ""] # [doc = " All of the caveats of `MaybeLiveLocals` apply."] pub struct MaybeTransitiveLiveLocals < 'a > { always_live : & 'a DenseBitSet < Local > , }
};
}
