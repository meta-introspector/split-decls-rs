macro_rules! COMMENT_START_SYMBOL {
    () => {
        # [doc = " `comment-start-symbol = %x23 ; #`"] pub (crate) const COMMENT_START_SYMBOL : u8 = b'#' ;
    };
}

COMMENT_START_SYMBOL!();