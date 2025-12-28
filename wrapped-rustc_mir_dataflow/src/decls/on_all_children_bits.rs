macro_rules! deps {
    () => {
        MoveData!();
    };
}

macro_rules! on_all_children_bits {
    () => {
        deps!();
        pub fn on_all_children_bits < 'tcx , F > (move_data : & MoveData < 'tcx > , move_path_index : MovePathIndex , mut each_child : F ,) where F : FnMut (MovePathIndex) , { fn on_all_children_bits < 'tcx , F > (move_data : & MoveData < 'tcx > , move_path_index : MovePathIndex , each_child : & mut F ,) where F : FnMut (MovePathIndex) , { each_child (move_path_index) ; let mut next_child_index = move_data . move_paths [move_path_index] . first_child ; while let Some (child_index) = next_child_index { on_all_children_bits (move_data , child_index , each_child) ; next_child_index = move_data . move_paths [child_index] . next_sibling ; } } on_all_children_bits (move_data , move_path_index , & mut each_child) ; }
    };
}

on_all_children_bits!();