macro_rules! deps {
    () => {
        MovePath!();
    };
}

macro_rules! new_move_path {
    () => {
        deps!();
        fn new_move_path < 'tcx > (move_paths : & mut IndexVec < MovePathIndex , MovePath < 'tcx > > , path_map : & mut IndexVec < MovePathIndex , SmallVec < [MoveOutIndex ; 4] > > , init_path_map : & mut IndexVec < MovePathIndex , SmallVec < [InitIndex ; 4] > > , parent : Option < MovePathIndex > , place : Place < 'tcx > ,) -> MovePathIndex { let move_path = move_paths . push (MovePath { next_sibling : None , first_child : None , parent , place }) ; if let Some (parent) = parent { let next_sibling = mem :: replace (& mut move_paths [parent] . first_child , Some (move_path)) ; move_paths [move_path] . next_sibling = next_sibling ; } let path_map_ent = path_map . push (smallvec ! []) ; assert_eq ! (path_map_ent , move_path) ; let init_path_map_ent = init_path_map . push (smallvec ! []) ; assert_eq ! (init_path_map_ent , move_path) ; move_path }
    };
}

new_move_path!()