macro_rules! WSCHAR {
    () => {
        # [doc = " ```bnf"] # [doc = " wschar =  %x20  ; Space"] # [doc = " wschar =/ %x09  ; Horizontal tab"] # [doc = " ```"] pub (crate) const WSCHAR : (u8 , u8) = (b' ' , b'\t') ;
    };
}

WSCHAR!();