macro_rules! deps {
    () => {
        AesReader!();
        AesMode!();
        ZipError!();
        AesReaderValid!();
        Cipher!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < R : Read > AesReader < R > { pub const fn new (reader : R , aes_mode : AesMode , compressed_size : u64) -> AesReader < R > { let data_length = compressed_size - (PWD_VERIFY_LENGTH + AUTH_CODE_LENGTH + aes_mode . salt_length ()) as u64 ; Self { reader , aes_mode , data_length , } } # [doc = " Read the AES header bytes and validate the password."] # [doc = ""] # [doc = " Even if the validation succeeds, there is still a 1 in 65536 chance that an incorrect"] # [doc = " password was provided."] # [doc = " It isn't possible to check the authentication code in this step. This will be done after"] # [doc = " reading and decrypting the file."] pub fn validate (mut self , password : & [u8]) -> Result < AesReaderValid < R > , ZipError > { let salt_length = self . aes_mode . salt_length () ; let key_length = self . aes_mode . key_length () ; let mut salt = vec ! [0 ; salt_length] ; self . reader . read_exact (& mut salt) ? ; let mut pwd_verification_value = vec ! [0 ; PWD_VERIFY_LENGTH] ; self . reader . read_exact (& mut pwd_verification_value) ? ; let derived_key_len = 2 * key_length + PWD_VERIFY_LENGTH ; let mut derived_key : Box < [u8] > = vec ! [0 ; derived_key_len] . into_boxed_slice () ; pbkdf2 :: pbkdf2 :: < Hmac < Sha1 > > (password , & salt , ITERATION_COUNT , & mut derived_key) . map_err (| e | Error :: new (ErrorKind :: InvalidInput , e)) ? ; let decrypt_key = & derived_key [0 .. key_length] ; let hmac_key = & derived_key [key_length .. key_length * 2] ; let pwd_verify = & derived_key [derived_key_len - 2 ..] ; if pwd_verification_value != pwd_verify { return Err (ZipError :: InvalidPassword) ; } let cipher = Cipher :: from_mode (self . aes_mode , decrypt_key) ; let hmac = Hmac :: < Sha1 > :: new_from_slice (hmac_key) . unwrap () ; Ok (AesReaderValid { reader : self . reader , data_remaining : self . data_length , cipher , hmac , finalized : false , }) } # [doc = " Read the AES header bytes and returns the verification value and salt."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " the verification value and the salt"] pub fn get_verification_value_and_salt (mut self ,) -> io :: Result < ([u8 ; PWD_VERIFY_LENGTH] , Vec < u8 >) > { let salt_length = self . aes_mode . salt_length () ; let mut salt = vec ! [0 ; salt_length] ; self . reader . read_exact (& mut salt) ? ; let mut pwd_verification_value = [0 ; PWD_VERIFY_LENGTH] ; self . reader . read_exact (& mut pwd_verification_value) ? ; Ok ((pwd_verification_value , salt)) } }
    };
}

impl_6!();