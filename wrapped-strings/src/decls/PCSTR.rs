macro_rules! PCSTR {
    () => {
        # [doc = " A pointer to a constant null-terminated string of 8-bit Windows (ANSI) characters."] # [repr (transparent)] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct PCSTR (pub * const u8) ;
    };
}

PCSTR!()