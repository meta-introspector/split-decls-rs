macro_rules! deps {
    () => {
        BlockType!();
        State!();
    };
}

macro_rules! zng_tr_stored_block {
    () => {
        deps!();
        pub (crate) fn zng_tr_stored_block (state : & mut State , window_range : core :: ops :: Range < usize > , is_last : bool ,) { state . bit_writer . emit_tree (BlockType :: StoredBlock , is_last) ; state . bit_writer . emit_align () ; state . bit_writer . cmpr_bits_align () ; let input_block : & [u8] = & state . window . filled () [window_range] ; let stored_len = input_block . len () as u16 ; state . bit_writer . pending . extend (& stored_len . to_le_bytes ()) ; state . bit_writer . pending . extend (& (! stored_len) . to_le_bytes ()) ; state . bit_writer . cmpr_bits_add (32) ; state . bit_writer . sent_bits_add (32) ; if stored_len > 0 { state . bit_writer . pending . extend (input_block) ; state . bit_writer . cmpr_bits_add ((stored_len << 3) as usize) ; state . bit_writer . sent_bits_add ((stored_len << 3) as usize) ; } }
    };
}

zng_tr_stored_block!()