macro_rules! deps {
    () => {
        State!();
        DeflateStream!();
        DataType!();
        TreeDesc!();
        Strategy!();
        BlockType!();
    };
}

macro_rules! zng_tr_flush_block {
    () => {
        deps!();
        fn zng_tr_flush_block (stream : & mut DeflateStream , window_offset : Option < usize > , stored_len : u32 , last : bool ,) { let mut opt_lenb ; let static_lenb ; let mut max_blindex = 0 ; let state = & mut stream . state ; if state . sym_buf . is_empty () { opt_lenb = 0 ; static_lenb = 0 ; state . static_len = 7 ; } else if state . level > 0 { if stream . data_type == DataType :: Unknown as i32 { stream . data_type = State :: detect_data_type (& state . l_desc . dyn_tree) as i32 ; } { let mut tmp = TreeDesc :: EMPTY ; core :: mem :: swap (& mut tmp , & mut state . l_desc) ; build_tree (state , & mut tmp) ; core :: mem :: swap (& mut tmp , & mut state . l_desc) ; trace ! ("\nlit data: dyn {}, stat {}" , state . opt_len , state . static_len) ; } { let mut tmp = TreeDesc :: EMPTY ; core :: mem :: swap (& mut tmp , & mut state . d_desc) ; build_tree (state , & mut tmp) ; core :: mem :: swap (& mut tmp , & mut state . d_desc) ; trace ! ("\ndist data: dyn {}, stat {}" , state . opt_len , state . static_len) ; } max_blindex = build_bl_tree (state) ; opt_lenb = (state . opt_len + 3 + 7) >> 3 ; static_lenb = (state . static_len + 3 + 7) >> 3 ; trace ! ("\nopt {}({}) stat {}({}) stored {} lit {} " , opt_lenb , state . opt_len , static_lenb , state . static_len , stored_len , state . sym_buf . len () / 3) ; if static_lenb <= opt_lenb || state . strategy == Strategy :: Fixed { opt_lenb = static_lenb ; } } else { assert ! (window_offset . is_some () , "lost buf") ; opt_lenb = stored_len as usize + 5 ; static_lenb = stored_len as usize + 5 ; } # [allow (clippy :: unnecessary_unwrap)] if stored_len as usize + 4 <= opt_lenb && window_offset . is_some () { let window_offset = window_offset . unwrap () ; let range = window_offset .. window_offset + stored_len as usize ; zng_tr_stored_block (state , range , last) ; } else if static_lenb == opt_lenb { state . bit_writer . emit_tree (BlockType :: StaticTrees , last) ; state . compress_block_static_trees () ; } else { state . bit_writer . emit_tree (BlockType :: DynamicTrees , last) ; send_all_trees (state , state . l_desc . max_code + 1 , state . d_desc . max_code + 1 , max_blindex + 1 ,) ; state . compress_block_dynamic_trees () ; } state . init_block () ; if last { state . bit_writer . emit_align () ; } }
    };
}

zng_tr_flush_block!();