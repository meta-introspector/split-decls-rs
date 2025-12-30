// Generated macro for new_body (function)
macro_rules! Depcrate_shimnew_body {
() => {
// Module: crate::shim
// Provides: {"new_body"}
// Dependencies: {}
fn new_body < 'tcx > (source : MirSource < 'tcx > , basic_blocks : IndexVec < BasicBlock , BasicBlockData < 'tcx > > , local_decls : IndexVec < Local , LocalDecl < 'tcx > > , arg_count : usize , span : Span ,) -> Body < 'tcx > { let mut body = Body :: new (source , basic_blocks , IndexVec :: from_elem_n (SourceScopeData { span , parent_scope : None , inlined : None , inlined_parent_scope : None , local_data : ClearCrossCrate :: Clear , } , 1 ,) , local_decls , IndexVec :: new () , arg_count , vec ! [] , span , None , None ,) ; body . set_required_consts (Vec :: new ()) ; body }
};
}
