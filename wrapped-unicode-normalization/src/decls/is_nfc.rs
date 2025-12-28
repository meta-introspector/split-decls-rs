macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfc {
    () => {
        deps!();
        # [doc = " Authoritatively check if a string is in NFC."] # [inline] pub fn is_nfc (s : & str) -> bool { match is_nfc_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . nfc ()) , } }
    };
}

is_nfc!();