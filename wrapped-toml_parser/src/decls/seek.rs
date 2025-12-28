macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! seek {
    () => {
        deps!();
        fn seek (stream : & mut Stream < '_ > , offset : isize) { let current = stream . checkpoint () ; stream . reset_to_start () ; let start = stream . checkpoint () ; let old_offset = current . offset_from (& start) ; let new_offset = (old_offset as isize) . saturating_add (offset) as usize ; if new_offset < stream . eof_offset () { # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (new_offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (new_offset) ; } else { stream . finish () ; } }
    };
}

seek!()