macro_rules! PSTR {
    () => {
        # [doc = " A pointer to a null-terminated string of 8-bit Windows (ANSI) characters."] # [repr (transparent)] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct PSTR (pub * mut u8) ;
    };
}

PSTR!();