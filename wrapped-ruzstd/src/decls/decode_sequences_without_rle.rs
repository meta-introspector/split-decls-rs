macro_rules! deps {
    () => {
        SequencesHeader!();
        BitReaderReversed!();
        Sequence!();
        DecodeSequenceError!();
        FSEScratch!();
        FSEDecoder!();
    };
}

macro_rules! decode_sequences_without_rle {
    () => {
        deps!();
        fn decode_sequences_without_rle (section : & SequencesHeader , br : & mut BitReaderReversed < '_ > , scratch : & FSEScratch , target : & mut Vec < Sequence > ,) -> Result < () , DecodeSequenceError > { let mut ll_dec = FSEDecoder :: new (& scratch . literal_lengths) ; let mut ml_dec = FSEDecoder :: new (& scratch . match_lengths) ; let mut of_dec = FSEDecoder :: new (& scratch . offsets) ; ll_dec . init_state (br) ? ; of_dec . init_state (br) ? ; ml_dec . init_state (br) ? ; target . clear () ; target . reserve (section . num_sequences as usize) ; for _seq_idx in 0 .. section . num_sequences { let ll_code = ll_dec . decode_symbol () ; let ml_code = ml_dec . decode_symbol () ; let of_code = of_dec . decode_symbol () ; let (ll_value , ll_num_bits) = lookup_ll_code (ll_code) ; let (ml_value , ml_num_bits) = lookup_ml_code (ml_code) ; if of_code > MAX_OFFSET_CODE { return Err (DecodeSequenceError :: UnsupportedOffset { offset_code : of_code , }) ; } let (obits , ml_add , ll_add) = br . get_bits_triple (of_code , ml_num_bits , ll_num_bits) ; let offset = obits as u32 + (1u32 << of_code) ; if offset == 0 { return Err (DecodeSequenceError :: ZeroOffset) ; } target . push (Sequence { ll : ll_value + ll_add as u32 , ml : ml_value + ml_add as u32 , of : offset , }) ; if target . len () < section . num_sequences as usize { ll_dec . update_state (br) ; ml_dec . update_state (br) ; of_dec . update_state (br) ; } if br . bits_remaining () < 0 { return Err (DecodeSequenceError :: NotEnoughBytesForNumSequences) ; } } if br . bits_remaining () > 0 { Err (DecodeSequenceError :: ExtraBits { bits_remaining : br . bits_remaining () , }) } else { Ok (()) } }
    };
}

decode_sequences_without_rle!()