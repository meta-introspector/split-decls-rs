// Generated macro for rustc (module)
macro_rules! Depcrate_maybe_transmutable_query_contextrustc {
() => {
// Module: crate::maybe_transmutable::query_context
// Provides: {"rustc"}
// Dependencies: {}
# [cfg (feature = "rustc")] mod rustc { use rustc_middle :: ty :: { Region , Ty , TyCtxt } ; use super :: * ; impl < 'tcx > super :: QueryContext for TyCtxt < 'tcx > { type Def = layout :: rustc :: Def < 'tcx > ; type Region = Region < 'tcx > ; type Type = Ty < 'tcx > ; } }
};
}
