// Generated macro for on_all_children_bits (function)
macro_rules! Depcrate_drop_flag_effectson_all_children_bits {
() => {
// Module: crate::drop_flag_effects
// Provides: {"on_all_children_bits"}
// Dependencies: {}
pub fn on_all_children_bits < 'tcx , F > (move_data : & MoveData < 'tcx > , move_path_index : MovePathIndex , mut each_child : F ,) where F : FnMut (MovePathIndex) , { fn on_all_children_bits < 'tcx , F > (move_data : & MoveData < 'tcx > , move_path_index : MovePathIndex , each_child : & mut F ,) where F : FnMut (MovePathIndex) , { each_child (move_path_index) ; let mut next_child_index = move_data . move_paths [move_path_index] . first_child ; while let Some (child_index) = next_child_index { on_all_children_bits (move_data , child_index , each_child) ; next_child_index = move_data . move_paths [child_index] . next_sibling ; } } on_all_children_bits (move_data , move_path_index , & mut each_child) ; }
};
}
