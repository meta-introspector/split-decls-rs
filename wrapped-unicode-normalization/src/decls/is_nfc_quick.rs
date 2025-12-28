macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfc_quick {
    () => {
        deps!();
        # [doc = " Quickly check if a string is in NFC, potentially returning"] # [doc = " `IsNormalized::Maybe` if further checks are necessary.  In this case a check"] # [doc = " like `s.chars().nfc().eq(s.chars())` should suffice."] # [inline] pub fn is_nfc_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfc , false) }
    };
}

is_nfc_quick!();