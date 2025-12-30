// Generated macro for drop_flag_effects_for_function_entry (function)
macro_rules! Depcrate_drop_flag_effectsdrop_flag_effects_for_function_entry {
() => {
// Module: crate::drop_flag_effects
// Provides: {"drop_flag_effects_for_function_entry"}
// Dependencies: {}
pub fn drop_flag_effects_for_function_entry < 'tcx , F > (body : & Body < 'tcx > , move_data : & MoveData < 'tcx > , mut callback : F ,) where F : FnMut (MovePathIndex , DropFlagState) , { for arg in body . args_iter () { let place = mir :: Place :: from (arg) ; let lookup_result = move_data . rev_lookup . find (place . as_ref ()) ; on_lookup_result_bits (move_data , lookup_result , | mpi | { callback (mpi , DropFlagState :: Present) }) ; } }
};
}
