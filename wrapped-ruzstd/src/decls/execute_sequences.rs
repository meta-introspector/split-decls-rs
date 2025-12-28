macro_rules! deps {
    () => {
        DecoderScratch!();
        ExecuteSequencesError!();
        Take!();
    };
}

macro_rules! execute_sequences {
    () => {
        deps!();
        # [doc = " Take the provided decoder and execute the sequences stored within"] pub fn execute_sequences (scratch : & mut DecoderScratch) -> Result < () , ExecuteSequencesError > { let mut literals_copy_counter = 0 ; let old_buffer_size = scratch . buffer . len () ; let mut seq_sum = 0 ; for idx in 0 .. scratch . sequences . len () { let seq = scratch . sequences [idx] ; if seq . ll > 0 { let high = literals_copy_counter + seq . ll as usize ; if high > scratch . literals_buffer . len () { return Err (ExecuteSequencesError :: NotEnoughBytesForSequence { wanted : high , have : scratch . literals_buffer . len () , }) ; } let literals = & scratch . literals_buffer [literals_copy_counter .. high] ; literals_copy_counter += seq . ll as usize ; scratch . buffer . push (literals) ; } let actual_offset = do_offset_history (seq . of , seq . ll , & mut scratch . offset_hist) ; if actual_offset == 0 { return Err (ExecuteSequencesError :: ZeroOffset) ; } if seq . ml > 0 { scratch . buffer . repeat (actual_offset as usize , seq . ml as usize) ? ; } seq_sum += seq . ml ; seq_sum += seq . ll ; } if literals_copy_counter < scratch . literals_buffer . len () { let rest_literals = & scratch . literals_buffer [literals_copy_counter ..] ; scratch . buffer . push (rest_literals) ; seq_sum += rest_literals . len () as u32 ; } let diff = scratch . buffer . len () - old_buffer_size ; assert ! (seq_sum as usize == diff , "Seq_sum: {} is different from the difference in buffersize: {}" , seq_sum , diff) ; Ok (()) }
    };
}

execute_sequences!()