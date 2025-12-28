macro_rules! PCWSTR {
    () => {
        # [doc = " A pointer to a constant null-terminated string of 16-bit Unicode characters."] # [repr (transparent)] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct PCWSTR (pub * const u16) ;
    };
}

PCWSTR!();