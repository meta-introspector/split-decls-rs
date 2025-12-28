macro_rules! is_ligature_transparent {
    () => {
        # [doc = " Whether this character is a default-ignorable combining mark"] # [doc = " or ZWJ. These characters won't interrupt non-Arabic ligatures."] fn is_ligature_transparent (c : char) -> bool { matches ! (c , '\u{34F}' | '\u{17B4}' ..='\u{17B5}' | '\u{180B}' ..='\u{180D}' | '\u{180F}' | '\u{200D}' | '\u{FE00}' ..='\u{FE0F}' | '\u{E0100}' ..='\u{E01EF}') }
    };
}

is_ligature_transparent!();