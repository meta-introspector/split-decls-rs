macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! from_char_iter {
    () => {
        deps!();
        # [inline] fn from_char_iter (iter : impl Iterator < Item = char >) -> SmolStr { from_buf_and_chars ([0 ; _] , 0 , iter) }
    };
}

from_char_iter!();