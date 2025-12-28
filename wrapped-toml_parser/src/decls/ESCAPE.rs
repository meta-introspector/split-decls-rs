macro_rules! ESCAPE {
    () => {
        # [doc = " `escape = %x5C                   ; \\`"] pub (crate) const ESCAPE : u8 = b'\\' ;
    };
}

ESCAPE!()