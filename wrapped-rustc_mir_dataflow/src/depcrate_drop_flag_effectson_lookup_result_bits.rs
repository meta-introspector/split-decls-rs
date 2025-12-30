// Generated macro for on_lookup_result_bits (function)
macro_rules! Depcrate_drop_flag_effectson_lookup_result_bits {
() => {
// Module: crate::drop_flag_effects
// Provides: {"on_lookup_result_bits"}
// Dependencies: {}
pub fn on_lookup_result_bits < 'tcx , F > (move_data : & MoveData < 'tcx > , lookup_result : LookupResult , each_child : F ,) where F : FnMut (MovePathIndex) , { match lookup_result { LookupResult :: Parent (..) => { } LookupResult :: Exact (e) => on_all_children_bits (move_data , e , each_child) , } }
};
}
