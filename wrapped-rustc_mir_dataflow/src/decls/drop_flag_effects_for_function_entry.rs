macro_rules! deps {
    () => {
        MoveData!();
        DropFlagState!();
    };
}

macro_rules! drop_flag_effects_for_function_entry {
    () => {
        deps!();
        pub fn drop_flag_effects_for_function_entry < 'tcx , F > (body : & Body < 'tcx > , move_data : & MoveData < 'tcx > , mut callback : F ,) where F : FnMut (MovePathIndex , DropFlagState) , { for arg in body . args_iter () { let place = mir :: Place :: from (arg) ; let lookup_result = move_data . rev_lookup . find (place . as_ref ()) ; on_lookup_result_bits (move_data , lookup_result , | mpi | { callback (mpi , DropFlagState :: Present) }) ; } }
    };
}

drop_flag_effects_for_function_entry!();