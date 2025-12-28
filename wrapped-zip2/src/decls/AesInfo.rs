macro_rules! deps {
    () => {
        AesMode!();
    };
}

macro_rules! AesInfo {
    () => {
        deps!();
        # [doc = " Holds the AES information of a file in the zip archive"] # [derive (Debug)] # [cfg (feature = "aes-crypto")] pub struct AesInfo { # [doc = " The AES encryption mode"] pub aes_mode : AesMode , # [doc = " The verification key"] pub verification_value : [u8 ; PWD_VERIFY_LENGTH] , # [doc = " The salt"] pub salt : Vec < u8 > , }
    };
}

AesInfo!()