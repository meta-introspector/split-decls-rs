macro_rules! deps {
    () => {
        State!();
        StaticTreeDesc!();
        TreeDesc!();
    };
}

macro_rules! build_bl_tree {
    () => {
        deps!();
        # [doc = " Construct the Huffman tree for the bit lengths and return the index in"] # [doc = " bl_order of the last bit length code to send."] fn build_bl_tree (state : & mut State) -> usize { scan_tree (& mut state . bl_desc , & mut state . l_desc . dyn_tree , state . l_desc . max_code ,) ; scan_tree (& mut state . bl_desc , & mut state . d_desc . dyn_tree , state . d_desc . max_code ,) ; { let mut tmp = TreeDesc :: EMPTY ; core :: mem :: swap (& mut tmp , & mut state . bl_desc) ; build_tree (state , & mut tmp) ; core :: mem :: swap (& mut tmp , & mut state . bl_desc) ; } let mut max_blindex = BL_CODES - 1 ; while max_blindex >= 3 { let index = StaticTreeDesc :: BL_ORDER [max_blindex] as usize ; if state . bl_desc . dyn_tree [index] . len () != 0 { break ; } max_blindex -= 1 ; } state . opt_len += 3 * (max_blindex + 1) + 5 + 5 + 4 ; trace ! ("\ndyn trees: dyn {}, stat {}" , state . opt_len , state . static_len) ; max_blindex }
    };
}

build_bl_tree!();