// Generated macro for impl_259 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_259 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_259"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for MonoItem < 'tcx > { type T = crate :: mir :: mono :: MonoItem ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: mir :: mono :: MonoItem as StableMonoItem ; match self { MonoItem :: Fn (instance) => StableMonoItem :: Fn (instance . stable (tables , cx)) , MonoItem :: Static (def_id) => StableMonoItem :: Static (tables . static_def (* def_id)) , MonoItem :: GlobalAsm (item_id) => StableMonoItem :: GlobalAsm (opaque (item_id)) , } } }
};
}
