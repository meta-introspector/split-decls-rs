// Generated macro for impl_28 (impl)
macro_rules! Depcrate_aesimpl_28 {
() => {
// Module: crate::aes
// Provides: {"impl_28"}
// Dependencies: {}
impl < W : Write > Write for AesWriter < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . write_encrypted_file_header () ? ; self . buffer . extend_from_slice (buf) ; self . cipher . crypt_in_place (& mut self . buffer [..]) ; self . hmac . update (& self . buffer [..]) ; self . writer . write_all (& self . buffer [..]) ? ; self . buffer . zeroize () ; self . buffer . clear () ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { self . writer . flush () } }
};
}
