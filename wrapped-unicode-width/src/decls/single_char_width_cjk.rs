macro_rules! single_char_width_cjk {
    () => {
        # [doc = " Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`, or"] # [doc = " `None` if `c` is a control character."] # [doc = " Ambiguous width characters are treated as wide."] # [cfg (feature = "cjk")] # [inline] pub fn single_char_width_cjk (c : char) -> Option < usize > { if c < '\u{7F}' { if c >= '\u{20}' { Some (1) } else { None } } else if c >= '\u{A0}' { Some (lookup_width_cjk (c) . 0 . into ()) } else { None } }
    };
}

single_char_width_cjk!();