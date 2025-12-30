// Generated macro for impl_226 (impl)
macro_rules! Depcrate_unstable_convert_stable_mirimpl_226 {
() => {
// Module: crate::unstable::convert::stable::mir
// Provides: {"impl_226"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for mir :: VarDebugInfoContents < 'tcx > { type T = crate :: mir :: VarDebugInfoContents ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { mir :: VarDebugInfoContents :: Place (place) => { crate :: mir :: VarDebugInfoContents :: Place (place . stable (tables , cx)) } mir :: VarDebugInfoContents :: Const (const_operand) => { let op = ConstOperand { span : const_operand . span . stable (tables , cx) , user_ty : const_operand . user_ty . map (| index | index . as_usize ()) , const_ : const_operand . const_ . stable (tables , cx) , } ; crate :: mir :: VarDebugInfoContents :: Const (op) } } } }
};
}
