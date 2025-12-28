macro_rules! basic_unescaped {
    () => {
        # [doc = " `basic-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii`"] fn basic_unescaped < 'i > (stream : & mut & 'i str) -> & 'i str { let offset = stream . as_bytes () . offset_for (| b | ! BASIC_UNESCAPED . contains_token (b)) . unwrap_or (stream . len ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) }
    };
}

basic_unescaped!()