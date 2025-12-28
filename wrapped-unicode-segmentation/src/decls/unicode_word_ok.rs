macro_rules! unicode_word_ok {
    () => {
        # [inline] fn unicode_word_ok (t : & (usize , & str)) -> bool { has_alphanumeric (& t . 1) }
    };
}

unicode_word_ok!()