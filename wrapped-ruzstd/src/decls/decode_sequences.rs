macro_rules! deps {
    () => {
        BitReaderReversed!();
        FSEScratch!();
        Sequence!();
        SequencesHeader!();
        DecodeSequenceError!();
    };
}

macro_rules! decode_sequences {
    () => {
        deps!();
        # [doc = " Decode the provided source as a series of sequences into the supplied `target`."] pub fn decode_sequences (section : & SequencesHeader , source : & [u8] , scratch : & mut FSEScratch , target : & mut Vec < Sequence > ,) -> Result < () , DecodeSequenceError > { let bytes_read = maybe_update_fse_tables (section , source , scratch) ? ; vprintln ! ("Updating tables used {} bytes" , bytes_read) ; let bit_stream = & source [bytes_read ..] ; let mut br = BitReaderReversed :: new (bit_stream) ; let mut skipped_bits = 0 ; loop { let val = br . get_bits (1) ; skipped_bits += 1 ; if val == 1 || skipped_bits > 8 { break ; } } if skipped_bits > 8 { return Err (DecodeSequenceError :: ExtraPadding { skipped_bits }) ; } if scratch . ll_rle . is_some () || scratch . ml_rle . is_some () || scratch . of_rle . is_some () { decode_sequences_with_rle (section , & mut br , scratch , target) } else { decode_sequences_without_rle (section , & mut br , scratch , target) } }
    };
}

decode_sequences!()