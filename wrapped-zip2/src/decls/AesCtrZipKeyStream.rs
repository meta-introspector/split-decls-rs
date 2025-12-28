macro_rules! deps {
    () => {
        Cipher!();
        AesKind!();
    };
}

macro_rules! AesCtrZipKeyStream {
    () => {
        deps!();
        # [doc = " An AES-CTR key stream generator."] # [doc = ""] # [doc = " Implements the slightly non-standard AES-CTR variant used by WinZip AES encryption."] # [doc = ""] # [doc = " Typical AES-CTR implementations combine a nonce with a 64 bit counter. WinZIP AES instead uses"] # [doc = " no nonce and also uses a different byte order (little endian) than NIST (big endian)."] # [doc = ""] # [doc = " The stream implements the `Read` trait; encryption or decryption is performed by XOR-ing the"] # [doc = " bytes from the key stream with the ciphertext/plaintext."] pub struct AesCtrZipKeyStream < C : AesKind > { # [doc = " Current AES counter."] counter : u128 , # [doc = " AES cipher instance."] cipher : C :: Cipher , # [doc = " Stores the currently available keystream bytes."] buffer : [u8 ; AES_BLOCK_SIZE] , # [doc = " Number of bytes already used up from `buffer`."] pos : usize , }
    };
}

AesCtrZipKeyStream!()