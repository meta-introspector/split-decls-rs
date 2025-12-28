macro_rules! deps {
    () => {
        SourceFile!();
        SpanEncoder!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < S : SpanEncoder > Encodable < S > for SourceFile { fn encode (& self , s : & mut S) { self . name . encode (s) ; self . src_hash . encode (s) ; self . checksum_hash . encode (s) ; self . source_len . encode (s) ; assert ! (self . lines . read () . is_lines ()) ; let lines = self . lines () ; s . emit_u32 (lines . len () as u32) ; if lines . len () != 0 { let max_line_length = if lines . len () == 1 { 0 } else { lines . array_windows () . map (| & [fst , snd] | snd - fst) . map (| bp | bp . to_usize ()) . max () . unwrap () } ; let bytes_per_diff : usize = match max_line_length { 0 ..= 0xFF => 1 , 0x100 ..= 0xFFFF => 2 , _ => 4 , } ; s . emit_u8 (bytes_per_diff as u8) ; assert_eq ! (lines [0] , RelativeBytePos (0)) ; let diff_iter = lines . array_windows () . map (| & [fst , snd] | snd - fst) ; let num_diffs = lines . len () - 1 ; let mut raw_diffs ; match bytes_per_diff { 1 => { raw_diffs = Vec :: with_capacity (num_diffs) ; for diff in diff_iter { raw_diffs . push (diff . 0 as u8) ; } } 2 => { raw_diffs = Vec :: with_capacity (bytes_per_diff * num_diffs) ; for diff in diff_iter { raw_diffs . extend_from_slice (& (diff . 0 as u16) . to_le_bytes ()) ; } } 4 => { raw_diffs = Vec :: with_capacity (bytes_per_diff * num_diffs) ; for diff in diff_iter { raw_diffs . extend_from_slice (& (diff . 0) . to_le_bytes ()) ; } } _ => unreachable ! () , } s . emit_raw_bytes (& raw_diffs) ; } self . multibyte_chars . encode (s) ; self . stable_id . encode (s) ; self . normalized_pos . encode (s) ; self . cnum . encode (s) ; } }
    };
}

impl_308!()