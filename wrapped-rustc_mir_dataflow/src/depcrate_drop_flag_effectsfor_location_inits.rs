// Generated macro for for_location_inits (function)
macro_rules! Depcrate_drop_flag_effectsfor_location_inits {
() => {
// Module: crate::drop_flag_effects
// Provides: {"for_location_inits"}
// Dependencies: {}
fn for_location_inits < 'tcx , F > (move_data : & MoveData < 'tcx > , loc : Location , mut callback : F) where F : FnMut (MovePathIndex) , { for ii in & move_data . init_loc_map [loc] { let init = move_data . inits [* ii] ; match init . kind { InitKind :: Deep => { let path = init . path ; on_all_children_bits (move_data , path , & mut callback) } InitKind :: Shallow => { let mpi = init . path ; callback (mpi) ; } InitKind :: NonPanicPathOnly => () , } } }
};
}
