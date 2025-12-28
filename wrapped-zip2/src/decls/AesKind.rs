macro_rules! deps {
    () => {
        Cipher!();
    };
}

macro_rules! AesKind {
    () => {
        deps!();
        # [doc = " An AES cipher kind."] pub trait AesKind { # [doc = " Key type."] type Key : AsRef < [u8] > ; # [doc = " Cipher used to decrypt."] type Cipher : KeyInit ; }
    };
}

AesKind!()