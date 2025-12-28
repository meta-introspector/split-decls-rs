macro_rules! is_incb_linker {
    () => {
        pub fn is_incb_linker (c : char) -> bool { matches ! (c , | '\u{94D}' | '\u{9CD}' | '\u{ACD}' | '\u{B4D}' | '\u{C4D}' | '\u{D4D}') }
    };
}

is_incb_linker!();