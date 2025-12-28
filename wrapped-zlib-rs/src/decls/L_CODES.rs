macro_rules! L_CODES {
    () => {
        # [doc = " number of Literal or Length codes, including the END_BLOCK code"] pub (crate) const L_CODES : usize = LITERALS + 1 + LENGTH_CODES ;
    };
}

L_CODES!();