// Generated macro for read_text (function)
macro_rules! Depcrate_compiler_msvcread_text {
() => {
// Module: crate::compiler::msvc
// Provides: {"read_text"}
// Dependencies: {}
# [doc = " Reads the text stream as a unicode buffer, prioritizing UTF-8, UTF-16 (big and little endian), and falling back on ISO 8859-1."] fn read_text < R > (reader : & mut R) -> io :: Result < String > where R : Read , { let mut buf = Vec :: new () ; reader . read_to_end (& mut buf) ? ; let (result , _ , has_error) = encoding_rs :: WINDOWS_1252 . decode (& buf) ; if has_error { Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "failed to decode text" ,)) } else { Ok (result . to_string ()) } }
};
}
