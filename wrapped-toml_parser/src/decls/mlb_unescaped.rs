macro_rules! mlb_unescaped {
    () => {
        # [doc = " `mlb-unescaped` extended with `mlb-quotes` and `LF`"] # [doc = ""] # [doc = " **warning:** `newline` is not validated"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ml-basic-body = *mlb-content *( mlb-quotes 1*mlb-content ) [ mlb-quotes ]"] # [doc = ""] # [doc = " mlb-content = mlb-char / newline / mlb-escaped-nl"] # [doc = " mlb-char = mlb-unescaped / escaped"] # [doc = " mlb-quotes = 1*2quotation-mark"] # [doc = " mlb-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii"] # [doc = " ```"] fn mlb_unescaped < 'i > (stream : & mut & 'i str) -> & 'i str { let offset = stream . as_bytes () . offset_for (| b | ! (MLB_UNESCAPED , b'"' , b'\n') . contains_token (b)) . unwrap_or (stream . len ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) }
    };
}

mlb_unescaped!()