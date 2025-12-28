macro_rules! deps {
    () => {
        AesReaderValid!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < R : Read > Read for AesReaderValid < R > { # [doc = " This implementation does not fulfill all requirements set in the trait documentation."] # [doc = ""] # [doc = " ```txt"] # [doc = " \"If an error is returned then it must be guaranteed that no bytes were read.\""] # [doc = " ```"] # [doc = ""] # [doc = " Whether this applies to errors that occur while reading the encrypted data depends on the"] # [doc = " underlying reader. If the error occurs while verifying the HMAC, the reader might become"] # [doc = " practically unusable, since its position after the error is not known."] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . data_remaining == 0 { return Ok (0) ; } let bytes_to_read = self . data_remaining . min (buf . len () as u64) as usize ; let read = self . reader . read (& mut buf [0 .. bytes_to_read]) ? ; self . data_remaining -= read as u64 ; self . hmac . update (& buf [0 .. read]) ; self . cipher . crypt_in_place (& mut buf [0 .. read]) ; if self . data_remaining == 0 { assert ! (! self . finalized , "Tried to use an already finalized HMAC. This is a bug!") ; self . finalized = true ; let mut read_auth_code = [0 ; AUTH_CODE_LENGTH] ; self . reader . read_exact (& mut read_auth_code) ? ; let computed_auth_code = & self . hmac . finalize_reset () . into_bytes () [0 .. AUTH_CODE_LENGTH] ; if ! constant_time_eq (computed_auth_code , & read_auth_code) { return Err (Error :: new (ErrorKind :: InvalidData , "Invalid authentication code, this could be due to an invalid password or errors in the data")) ; } } Ok (read) } }
    };
}

impl_8!();