macro_rules! WANT_MIN_MATCH {
    () => {
        # [doc = " The minimum wanted match length, affects deflate_quick, deflate_fast, deflate_medium and deflate_slow"] pub (crate) const WANT_MIN_MATCH : usize = 4 ;
    };
}

WANT_MIN_MATCH!()