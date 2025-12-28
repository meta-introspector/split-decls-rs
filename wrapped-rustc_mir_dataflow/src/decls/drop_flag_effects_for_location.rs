macro_rules! deps {
    () => {
        LookupResult!();
        MoveData!();
        DropFlagState!();
    };
}

macro_rules! drop_flag_effects_for_location {
    () => {
        deps!();
        pub fn drop_flag_effects_for_location < 'tcx , F > (body : & Body < 'tcx > , move_data : & MoveData < 'tcx > , loc : Location , mut callback : F ,) where F : FnMut (MovePathIndex , DropFlagState) , { debug ! ("drop_flag_effects_for_location({:?})" , loc) ; for mi in & move_data . loc_map [loc] { let path = mi . move_path_index (move_data) ; debug ! ("moving out of path {:?}" , move_data . move_paths [path]) ; on_all_children_bits (move_data , path , | mpi | callback (mpi , DropFlagState :: Absent)) } if let Some (Terminator { kind : TerminatorKind :: Drop { place , .. } , .. }) = body . stmt_at (loc) . right () && let LookupResult :: Exact (mpi) = move_data . rev_lookup . find (place . as_ref ()) { on_all_children_bits (move_data , mpi , | mpi | callback (mpi , DropFlagState :: Absent)) } debug ! ("drop_flag_effects: assignment for location({:?})" , loc) ; for_location_inits (move_data , loc , | mpi | callback (mpi , DropFlagState :: Present)) ; }
    };
}

drop_flag_effects_for_location!()