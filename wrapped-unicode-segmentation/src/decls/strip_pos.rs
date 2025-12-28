macro_rules! strip_pos {
    () => {
        # [inline (always)] fn strip_pos ((_ , w) : (usize , & str)) -> & str { w }
    };
}

strip_pos!()