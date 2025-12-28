macro_rules! is_hangul_syllable {
    () => {
        pub (crate) fn is_hangul_syllable (c : char) -> bool { (c as u32) >= S_BASE && (c as u32) < (S_BASE + S_COUNT) }
    };
}

is_hangul_syllable!()