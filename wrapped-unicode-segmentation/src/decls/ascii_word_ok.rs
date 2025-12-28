macro_rules! ascii_word_ok {
    () => {
        # [inline] fn ascii_word_ok (t : & (usize , & str)) -> bool { has_ascii_alphanumeric (& t . 1) }
    };
}

ascii_word_ok!()