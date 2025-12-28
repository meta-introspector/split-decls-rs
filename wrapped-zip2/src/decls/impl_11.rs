macro_rules! deps {
    () => {
        AesMode!();
        AesWriter!();
        Cipher!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < W : Write > AesWriter < W > { pub fn new (writer : W , aes_mode : AesMode , password : & [u8]) -> io :: Result < Self > { let salt_length = aes_mode . salt_length () ; let key_length = aes_mode . key_length () ; let mut encrypted_file_header = Vec :: with_capacity (salt_length + 2) ; let mut salt = vec ! [0 ; salt_length] ; getrandom :: fill (& mut salt) ? ; encrypted_file_header . write_all (& salt) ? ; let derived_key_len = 2 * key_length + PWD_VERIFY_LENGTH ; let mut derived_key : Zeroizing < Vec < u8 > > = Zeroizing :: new (vec ! [0 ; derived_key_len]) ; pbkdf2 :: pbkdf2 :: < Hmac < Sha1 > > (password , & salt , ITERATION_COUNT , & mut derived_key) . map_err (| e | Error :: new (ErrorKind :: InvalidInput , e)) ? ; let encryption_key = & derived_key [0 .. key_length] ; let hmac_key = & derived_key [key_length .. key_length * 2] ; let pwd_verify = derived_key [derived_key_len - 2 ..] . to_vec () ; encrypted_file_header . write_all (& pwd_verify) ? ; let cipher = Cipher :: from_mode (aes_mode , encryption_key) ; let hmac = Hmac :: < Sha1 > :: new_from_slice (hmac_key) . unwrap () ; Ok (Self { writer , cipher , hmac , buffer : Default :: default () , encrypted_file_header : Some (encrypted_file_header) , }) } pub fn finish (mut self) -> io :: Result < W > { self . write_encrypted_file_header () ? ; let computed_auth_code = & self . hmac . finalize_reset () . into_bytes () [0 .. AUTH_CODE_LENGTH] ; self . writer . write_all (computed_auth_code) ? ; Ok (self . writer) } # [doc = " The AES encryption specification requires some metadata being written at the start of the"] # [doc = " file data section, but this can only be done once the extra data writing has been finished"] # [doc = " so we can't do it when the writer is constructed."] fn write_encrypted_file_header (& mut self) -> io :: Result < () > { if let Some (header) = self . encrypted_file_header . take () { self . writer . write_all (& header) ? ; } Ok (()) } }
    };
}

impl_11!();