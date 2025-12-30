// Generated macro for impl_238 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_238 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_238"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: ConstOperand < 'tcx > { type T = crate :: mir :: ConstOperand ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: mir :: ConstOperand { span : self . span . stable (tables , cx) , user_ty : self . user_ty . map (| u | u . as_usize ()) . or (None) , const_ : self . const_ . stable (tables , cx) , } } }
};
}
