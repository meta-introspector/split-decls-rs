macro_rules! basic_invalid {
    () => {
        fn basic_invalid < 'i > (stream : & mut & 'i str) -> & 'i str { let offset = stream . as_bytes () . offset_for (| b | (BASIC_UNESCAPED , ESCAPE) . contains_token (b)) . unwrap_or (stream . len ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) }
    };
}

basic_invalid!();