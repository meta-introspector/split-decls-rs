// Generated macro for impl_287 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_287 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_287"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: BoundRegionKind { type T = crate :: ty :: BoundRegionKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: BoundRegionKind ; match self { ty :: BoundRegionKind :: Anon => BoundRegionKind :: BrAnon , ty :: BoundRegionKind :: Named (def_id) => BoundRegionKind :: BrNamed (tables . br_named_def (* def_id) , cx . tcx . item_name (* def_id) . to_string () ,) , ty :: BoundRegionKind :: ClosureEnv => BoundRegionKind :: BrEnv , ty :: BoundRegionKind :: NamedAnon (_) => bug ! ("only used for pretty printing") , } } }
};
}
