macro_rules! deps {
    () => {
        AesKind!();
        AesCtrZipKeyStream!();
        AesCipher!();
        Cipher!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < C > AesCipher for AesCtrZipKeyStream < C > where C : AesKind , C :: Cipher : BlockEncrypt , { # [doc = " Decrypt or encrypt `target`."] # [inline] fn crypt_in_place (& mut self , mut target : & mut [u8]) { while ! target . is_empty () { if self . pos == AES_BLOCK_SIZE { self . buffer . as_mut () . write_u128_le (self . counter) . expect ("did not expect u128 le conversion to fail") ; self . cipher . encrypt_block (GenericArray :: from_mut_slice (& mut self . buffer)) ; self . counter += 1 ; self . pos = 0 ; } let target_len = target . len () . min (AES_BLOCK_SIZE - self . pos) ; xor (& mut target [0 .. target_len] , & self . buffer [self . pos .. (self . pos + target_len)] ,) ; target = & mut target [target_len ..] ; self . pos += target_len ; } } }
    };
}

impl_26!()