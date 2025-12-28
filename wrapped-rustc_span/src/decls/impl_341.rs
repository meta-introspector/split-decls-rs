macro_rules! deps {
    () => {
        Span!();
        SourceMap!();
        SourceFile!();
        HashStableContext!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for Span where CTX : HashStableContext , { # [doc = " Hashes a span in a stable way. We can't directly hash the span's `BytePos`"] # [doc = " fields (that would be similar to hashing pointers, since those are just"] # [doc = " offsets into the `SourceMap`). Instead, we hash the (file name, line, column)"] # [doc = " triple, which stays the same even if the containing `SourceFile` has moved"] # [doc = " within the `SourceMap`."] # [doc = ""] # [doc = " Also note that we are hashing byte offsets for the column, not unicode"] # [doc = " codepoint offsets. For the purpose of the hash that's sufficient."] # [doc = " Also, hashing filenames is expensive so we avoid doing it twice when the"] # [doc = " span starts and ends in the same file, which is almost always the case."] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { const TAG_VALID_SPAN : u8 = 0 ; const TAG_INVALID_SPAN : u8 = 1 ; const TAG_RELATIVE_SPAN : u8 = 2 ; if ! ctx . hash_spans () { return ; } let span = self . data_untracked () ; span . ctxt . hash_stable (ctx , hasher) ; span . parent . hash_stable (ctx , hasher) ; if span . is_dummy () { Hash :: hash (& TAG_INVALID_SPAN , hasher) ; return ; } if let Some (parent) = span . parent { let def_span = ctx . def_span (parent) . data_untracked () ; if def_span . contains (span) { Hash :: hash (& TAG_RELATIVE_SPAN , hasher) ; (span . lo - def_span . lo) . to_u32 () . hash_stable (ctx , hasher) ; (span . hi - def_span . lo) . to_u32 () . hash_stable (ctx , hasher) ; return ; } } let Some ((file , line_lo , col_lo , line_hi , col_hi)) = ctx . span_data_to_lines_and_cols (& span) else { Hash :: hash (& TAG_INVALID_SPAN , hasher) ; return ; } ; Hash :: hash (& TAG_VALID_SPAN , hasher) ; Hash :: hash (& file , hasher) ; let col_lo_trunc = (col_lo . 0 as u64) & 0xFF ; let line_lo_trunc = ((line_lo as u64) & 0xFF_FF_FF) << 8 ; let col_hi_trunc = (col_hi . 0 as u64) & 0xFF << 32 ; let line_hi_trunc = ((line_hi as u64) & 0xFF_FF_FF) << 40 ; let col_line = col_lo_trunc | line_lo_trunc | col_hi_trunc | line_hi_trunc ; let len = (span . hi - span . lo) . 0 ; Hash :: hash (& col_line , hasher) ; Hash :: hash (& len , hasher) ; } }
    };
}

impl_341!();