macro_rules! deps {
    () => {
        MoveData!();
        InitKind!();
    };
}

macro_rules! for_location_inits {
    () => {
        deps!();
        fn for_location_inits < 'tcx , F > (move_data : & MoveData < 'tcx > , loc : Location , mut callback : F) where F : FnMut (MovePathIndex) , { for ii in & move_data . init_loc_map [loc] { let init = move_data . inits [* ii] ; match init . kind { InitKind :: Deep => { let path = init . path ; on_all_children_bits (move_data , path , & mut callback) } InitKind :: Shallow => { let mpi = init . path ; callback (mpi) ; } InitKind :: NonPanicPathOnly => () , } } }
    };
}

for_location_inits!()