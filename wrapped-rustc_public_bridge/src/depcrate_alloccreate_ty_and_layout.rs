// Generated macro for create_ty_and_layout (function)
macro_rules! Depcrate_alloccreate_ty_and_layout {
() => {
// Module: crate::alloc
// Provides: {"create_ty_and_layout"}
// Dependencies: {}
pub fn create_ty_and_layout < 'tcx , B : Bridge > (cx : & CompilerCtxt < 'tcx , B > , ty : Ty < 'tcx > ,) -> Result < TyAndLayout < 'tcx , Ty < 'tcx > > , & 'tcx layout :: LayoutError < 'tcx > > { use crate :: context :: TypingEnvHelpers ; cx . tcx . layout_of (cx . fully_monomorphized () . as_query_input (ty)) }
};
}
