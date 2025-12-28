macro_rules! Bit {
    () => {
        # [doc = " The **marker trait** for compile time bits."] pub trait Bit : Sealed + Copy + Default + 'static { # [allow (missing_docs)] const U8 : u8 ; # [allow (missing_docs)] const BOOL : bool ; # [doc = " Instantiates a singleton representing this bit."] fn new () -> Self ; # [allow (missing_docs)] fn to_u8 () -> u8 ; # [allow (missing_docs)] fn to_bool () -> bool ; }
    };
}

Bit!();