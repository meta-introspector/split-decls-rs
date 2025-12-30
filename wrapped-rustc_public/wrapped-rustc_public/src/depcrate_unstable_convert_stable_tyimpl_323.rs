// Generated macro for impl_323 (impl)
macro_rules! Depcrate_unstable_convert_stable_tyimpl_323 {
() => {
// Module: crate::unstable::convert::stable::ty
// Provides: {"impl_323"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for ty :: AssocKind { type T = crate :: ty :: AssocKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: { AssocKind , AssocTypeData } ; match * self { ty :: AssocKind :: Const { name } => AssocKind :: Const { name : name . to_string () } , ty :: AssocKind :: Fn { name , has_self } => { AssocKind :: Fn { name : name . to_string () , has_self } } ty :: AssocKind :: Type { data } => AssocKind :: Type { data : match data { ty :: AssocTypeData :: Normal (name) => AssocTypeData :: Normal (name . to_string ()) , ty :: AssocTypeData :: Rpitit (rpitit) => { AssocTypeData :: Rpitit (rpitit . stable (tables , cx)) } } , } , } } }
};
}
