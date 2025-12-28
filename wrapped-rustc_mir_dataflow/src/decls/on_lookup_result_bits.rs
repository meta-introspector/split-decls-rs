macro_rules! deps {
    () => {
        MoveData!();
        LookupResult!();
    };
}

macro_rules! on_lookup_result_bits {
    () => {
        deps!();
        pub fn on_lookup_result_bits < 'tcx , F > (move_data : & MoveData < 'tcx > , lookup_result : LookupResult , each_child : F ,) where F : FnMut (MovePathIndex) , { match lookup_result { LookupResult :: Parent (..) => { } LookupResult :: Exact (e) => on_all_children_bits (move_data , e , each_child) , } }
    };
}

on_lookup_result_bits!();