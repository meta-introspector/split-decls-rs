// Generated macro for new_allocation (function)
macro_rules! Depcrate_mir_allocnew_allocation {
() => {
// Module: crate::mir::alloc
// Provides: {"new_allocation"}
// Dependencies: {}
# [allow (rustc :: usage_of_qualified_ty)] pub (crate) fn new_allocation < 'tcx > (ty : rustc_middle :: ty :: Ty < 'tcx > , const_value : ConstValue , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Allocation { try_new_allocation (ty , const_value , tables , cx) . unwrap_or_else (| _ | panic ! ("Failed to convert: {const_value:?} to {ty:?}")) }
};
}
