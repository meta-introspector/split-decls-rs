macro_rules! deps {
    () => {
        NormalizedPos!();
    };
}

macro_rules! normalize_newlines {
    () => {
        deps!();
        # [doc = " Replaces `\\r\\n` with `\\n` in-place in `src`."] # [doc = ""] # [doc = " Leaves any occurrences of lone `\\r` unchanged."] fn normalize_newlines (src : & mut String , normalized_pos : & mut Vec < NormalizedPos >) { if ! src . as_bytes () . contains (& b'\r') { return ; } let mut buf = std :: mem :: replace (src , String :: new ()) . into_bytes () ; let mut gap_len = 0 ; let mut tail = buf . as_mut_slice () ; let mut cursor = 0 ; let original_gap = normalized_pos . last () . map_or (0 , | l | l . diff) ; loop { let idx = match find_crlf (& tail [gap_len ..]) { None => tail . len () , Some (idx) => idx + gap_len , } ; tail . copy_within (gap_len .. idx , 0) ; tail = & mut tail [idx - gap_len ..] ; if tail . len () == gap_len { break ; } cursor += idx - gap_len ; gap_len += 1 ; normalized_pos . push (NormalizedPos { pos : RelativeBytePos :: from_usize (cursor + 1) , diff : original_gap + gap_len as u32 , }) ; } let new_len = buf . len () - gap_len ; unsafe { buf . set_len (new_len) ; * src = String :: from_utf8_unchecked (buf) ; } fn find_crlf (src : & [u8]) -> Option < usize > { let mut search_idx = 0 ; while let Some (idx) = find_cr (& src [search_idx ..]) { if src [search_idx ..] . get (idx + 1) != Some (& b'\n') { search_idx += idx + 1 ; continue ; } return Some (search_idx + idx) ; } None } fn find_cr (src : & [u8]) -> Option < usize > { src . iter () . position (| & b | b == b'\r') } }
    };
}

normalize_newlines!();