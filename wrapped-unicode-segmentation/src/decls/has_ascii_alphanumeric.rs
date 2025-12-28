macro_rules! has_ascii_alphanumeric {
    () => {
        # [inline] fn has_ascii_alphanumeric (s : & & str) -> bool { s . chars () . any (| c | c . is_ascii_alphanumeric ()) }
    };
}

has_ascii_alphanumeric!()