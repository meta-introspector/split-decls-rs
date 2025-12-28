macro_rules! deps {
    () => {
        Cipher!();
    };
}

macro_rules! AesWriter {
    () => {
        deps!();
        pub struct AesWriter < W > { writer : W , cipher : Cipher , hmac : Hmac < Sha1 > , buffer : Zeroizing < Vec < u8 > > , encrypted_file_header : Option < Vec < u8 > > , }
    };
}

AesWriter!()