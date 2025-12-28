macro_rules! deps {
    () => {
        BitWriter!();
        Sequence!();
        FSETable!();
        State!();
    };
}

macro_rules! encode_sequences {
    () => {
        deps!();
        fn encode_sequences (sequences : & [crate :: blocks :: sequence_section :: Sequence] , writer : & mut BitWriter < & mut Vec < u8 > > , ll_table : & FSETable , ml_table : & FSETable , of_table : & FSETable ,) { let sequence = sequences [sequences . len () - 1] ; let (ll_code , ll_add_bits , ll_num_bits) = encode_literal_length (sequence . ll) ; let (of_code , of_add_bits , of_num_bits) = encode_offset (sequence . of) ; let (ml_code , ml_add_bits , ml_num_bits) = encode_match_len (sequence . ml) ; let mut ll_state : & State = ll_table . start_state (ll_code) ; let mut ml_state : & State = ml_table . start_state (ml_code) ; let mut of_state : & State = of_table . start_state (of_code) ; writer . write_bits (ll_add_bits , ll_num_bits) ; writer . write_bits (ml_add_bits , ml_num_bits) ; writer . write_bits (of_add_bits , of_num_bits) ; if sequences . len () > 1 { for sequence in (0 ..= sequences . len () - 2) . rev () { let sequence = sequences [sequence] ; let (ll_code , ll_add_bits , ll_num_bits) = encode_literal_length (sequence . ll) ; let (of_code , of_add_bits , of_num_bits) = encode_offset (sequence . of) ; let (ml_code , ml_add_bits , ml_num_bits) = encode_match_len (sequence . ml) ; { let next = of_table . next_state (of_code , of_state . index) ; let diff = of_state . index - next . baseline ; writer . write_bits (diff as u64 , next . num_bits as usize) ; of_state = next ; } { let next = ml_table . next_state (ml_code , ml_state . index) ; let diff = ml_state . index - next . baseline ; writer . write_bits (diff as u64 , next . num_bits as usize) ; ml_state = next ; } { let next = ll_table . next_state (ll_code , ll_state . index) ; let diff = ll_state . index - next . baseline ; writer . write_bits (diff as u64 , next . num_bits as usize) ; ll_state = next ; } writer . write_bits (ll_add_bits , ll_num_bits) ; writer . write_bits (ml_add_bits , ml_num_bits) ; writer . write_bits (of_add_bits , of_num_bits) ; } } writer . write_bits (ml_state . index as u64 , ml_table . table_size . ilog2 () as usize) ; writer . write_bits (of_state . index as u64 , of_table . table_size . ilog2 () as usize) ; writer . write_bits (ll_state . index as u64 , ll_table . table_size . ilog2 () as usize) ; let bits_to_fill = writer . misaligned () ; if bits_to_fill == 0 { writer . write_bits (1u32 , 8) ; } else { writer . write_bits (1u32 , bits_to_fill) ; } }
    };
}

encode_sequences!()