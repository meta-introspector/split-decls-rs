macro_rules! deps {
    () => {
        AesWriter!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < W : Write > Write for AesWriter < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . write_encrypted_file_header () ? ; self . buffer . extend_from_slice (buf) ; self . cipher . crypt_in_place (& mut self . buffer [..]) ; self . hmac . update (& self . buffer [..]) ; self . writer . write_all (& self . buffer [..]) ? ; self . buffer . zeroize () ; self . buffer . clear () ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { self . writer . flush () } }
    };
}

impl_12!()