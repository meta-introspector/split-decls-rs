macro_rules! deps {
    () => {
        MoveData!();
    };
}

macro_rules! move_path_children_matching {
    () => {
        deps!();
        pub fn move_path_children_matching < 'tcx , F > (move_data : & MoveData < 'tcx > , path : MovePathIndex , mut cond : F ,) -> Option < MovePathIndex > where F : FnMut (mir :: PlaceElem < 'tcx >) -> bool , { let mut next_child = move_data . move_paths [path] . first_child ; while let Some (child_index) = next_child { let move_path_children = & move_data . move_paths [child_index] ; if let Some (& elem) = move_path_children . place . projection . last () { if cond (elem) { return Some (child_index) ; } } next_child = move_path_children . next_sibling ; } None }
    };
}

move_path_children_matching!();