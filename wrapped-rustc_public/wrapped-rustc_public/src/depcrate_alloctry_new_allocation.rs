// Generated macro for try_new_allocation (function)
macro_rules! Depcrate_alloctry_new_allocation {
() => {
// Module: crate::alloc
// Provides: {"try_new_allocation"}
// Dependencies: {}
# [allow (rustc :: usage_of_qualified_ty)] pub (crate) fn try_new_allocation < 'tcx > (ty : rustc_middle :: ty :: Ty < 'tcx > , const_value : ConstValue , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Result < Allocation , Error > { let layout = alloc :: create_ty_and_layout (cx , ty) . map_err (| e | Error :: from_internal (e)) ? ; match const_value { ConstValue :: Scalar (scalar) => { alloc :: try_new_scalar (layout , scalar , cx) . map (| alloc | alloc . stable (tables , cx)) } ConstValue :: ZeroSized => Ok (new_empty_allocation (layout . align . abi)) , ConstValue :: Slice { alloc_id , meta } => { alloc :: try_new_slice (layout , alloc_id , meta , cx) . map (| alloc | alloc . stable (tables , cx)) } ConstValue :: Indirect { alloc_id , offset } => { let alloc = alloc :: try_new_indirect (alloc_id , cx) ; use rustc_public_bridge :: context :: AllocRangeHelpers ; Ok (allocation_filter (& alloc . 0 , cx . alloc_range (offset , layout . size) , tables , cx)) } } }
};
}
