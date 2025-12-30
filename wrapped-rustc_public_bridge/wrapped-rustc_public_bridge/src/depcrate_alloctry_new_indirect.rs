// Generated macro for try_new_indirect (function)
macro_rules! Depcrate_alloctry_new_indirect {
() => {
// Module: crate::alloc
// Provides: {"try_new_indirect"}
// Dependencies: {}
pub fn try_new_indirect < 'tcx , B : Bridge > (alloc_id : AllocId , cx : & CompilerCtxt < 'tcx , B > ,) -> ConstAllocation < 'tcx > { let alloc = cx . tcx . global_alloc (alloc_id) . unwrap_memory () ; alloc }
};
}
