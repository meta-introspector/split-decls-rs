macro_rules! stream_safe_leading_nonstarters {
    () => {
        # [inline] pub fn stream_safe_leading_nonstarters (c : char) -> usize { match c { '\u{0340}' => 1 , '\u{0341}' => 1 , '\u{0343}' => 1 , '\u{0344}' => 2 , '\u{0F73}' => 2 , '\u{0F75}' => 2 , '\u{0F81}' => 2 , '\u{FF9E}' => 1 , '\u{FF9F}' => 1 , _ => 0 , } }
    };
}

stream_safe_leading_nonstarters!()