macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfc_stream_safe {
    () => {
        deps!();
        # [doc = " Authoritatively check if a string is Stream-Safe NFC."] # [inline] pub fn is_nfc_stream_safe (s : & str) -> bool { match is_nfc_stream_safe_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . stream_safe () . nfc ()) , } }
    };
}

is_nfc_stream_safe!()