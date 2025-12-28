macro_rules! deps {
    () => {
        InactiveVariants!();
        LookupResult!();
        MoveData!();
    };
}

macro_rules! on_all_inactive_variants {
    () => {
        deps!();
        # [doc = " Calls `handle_inactive_variant` for each child move path of `enum_place` corresponding to an"] # [doc = " inactive variant at a particular `SwitchInt` edge."] pub (crate) fn on_all_inactive_variants < 'tcx > (move_data : & MoveData < 'tcx > , enum_place : mir :: Place < 'tcx > , inactive_variants : & InactiveVariants , mut handle_inactive_variant : impl FnMut (MovePathIndex) ,) { let LookupResult :: Exact (enum_mpi) = move_data . rev_lookup . find (enum_place . as_ref ()) else { return ; } ; let enum_path = & move_data . move_paths [enum_mpi] ; for (variant_mpi , variant_path) in enum_path . children (& move_data . move_paths) { let (downcast , base_proj) = variant_path . place . projection . split_last () . unwrap () ; assert_eq ! (enum_place . projection . len () , base_proj . len ()) ; let mir :: ProjectionElem :: Downcast (_ , variant_idx) = * downcast else { unreachable ! () ; } ; if inactive_variants . contains (variant_idx) { on_all_children_bits (move_data , variant_mpi , | mpi | handle_inactive_variant (mpi)) ; } } }
    };
}

on_all_inactive_variants!()