macro_rules! deps {
    () => {
        State!();
        StaticTreeDesc!();
    };
}

macro_rules! send_all_trees {
    () => {
        deps!();
        fn send_all_trees (state : & mut State , lcodes : usize , dcodes : usize , blcodes : usize) { assert ! (lcodes >= 257 && dcodes >= 1 && blcodes >= 4 , "not enough codes") ; assert ! (lcodes <= L_CODES && dcodes <= D_CODES && blcodes <= BL_CODES , "too many codes") ; trace ! ("\nbl counts: ") ; state . bit_writer . send_bits (lcodes as u64 - 257 , 5) ; state . bit_writer . send_bits (dcodes as u64 - 1 , 5) ; state . bit_writer . send_bits (blcodes as u64 - 4 , 4) ; for rank in 0 .. blcodes { trace ! ("\nbl code {:>2} " , StaticTreeDesc :: BL_ORDER [rank]) ; state . bit_writer . send_bits (state . bl_desc . dyn_tree [StaticTreeDesc :: BL_ORDER [rank] as usize] . len () as u64 , 3 ,) ; } trace ! ("\nbl tree: sent {}" , state . bit_writer . bits_sent) ; state . bit_writer . send_tree (& state . l_desc . dyn_tree , & state . bl_desc . dyn_tree , lcodes - 1) ; trace ! ("\nlit tree: sent {}" , state . bit_writer . bits_sent) ; state . bit_writer . send_tree (& state . d_desc . dyn_tree , & state . bl_desc . dyn_tree , dcodes - 1) ; trace ! ("\ndist tree: sent {}" , state . bit_writer . bits_sent) ; }
    };
}

send_all_trees!()