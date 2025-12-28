macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfc_stream_safe_quick {
    () => {
        deps!();
        # [doc = " Quickly check if a string is Stream-Safe NFC."] # [inline] pub fn is_nfc_stream_safe_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfc , true) }
    };
}

is_nfc_stream_safe_quick!();