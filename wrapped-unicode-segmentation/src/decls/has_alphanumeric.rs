macro_rules! has_alphanumeric {
    () => {
        # [inline] fn has_alphanumeric (s : & & str) -> bool { use crate :: tables :: util :: is_alphanumeric ; s . chars () . any (is_alphanumeric) }
    };
}

has_alphanumeric!()