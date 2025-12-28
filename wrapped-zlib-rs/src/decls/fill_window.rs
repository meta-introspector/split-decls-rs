macro_rules! deps {
    () => {
        DeflateStream!();
    };
}

macro_rules! fill_window {
    () => {
        deps!();
        # [inline] pub (crate) fn fill_window (stream : & mut DeflateStream) { debug_assert ! (stream . state . lookahead < MIN_LOOKAHEAD) ; let wsize = stream . state . w_size ; loop { let state = & mut * stream . state ; let mut more = state . window_size - state . lookahead - state . strstart ; if state . strstart >= wsize + state . max_dist () { let (old , new) = state . window . filled_mut () [.. 2 * wsize] . split_at_mut (wsize) ; old . copy_from_slice (new) ; state . match_start = state . match_start . saturating_sub (wsize as u16) ; if state . match_start == 0 { state . prev_length = 0 ; } state . strstart -= wsize ; state . block_start -= wsize as isize ; state . insert = Ord :: min (state . insert , state . strstart) ; self :: slide_hash :: slide_hash (state) ; more += wsize ; } if stream . avail_in == 0 { break ; } assert ! (more >= 2 , "more < 2") ; let n = read_buf_window (stream , stream . state . strstart + stream . state . lookahead , more) ; let state = & mut * stream . state ; state . lookahead += n ; if state . lookahead + state . insert >= STD_MIN_MATCH { let string = state . strstart - state . insert ; if state . max_chain_length > 1024 { let v0 = state . window . filled () [string] as u32 ; let v1 = state . window . filled () [string + 1] as u32 ; state . ins_h = state . update_hash (v0 , v1) ; } else if string >= 1 { state . quick_insert_string (string + 2 - STD_MIN_MATCH) ; } let mut count = state . insert ; if state . lookahead == 1 { count -= 1 ; } if count > 0 { state . insert_string (string , count) ; state . insert -= count ; } } if ! (stream . state . lookahead < MIN_LOOKAHEAD && stream . avail_in != 0) { break ; } } assert ! (stream . state . strstart <= stream . state . window_size - MIN_LOOKAHEAD , "not enough room for search") ; }
    };
}

fill_window!()