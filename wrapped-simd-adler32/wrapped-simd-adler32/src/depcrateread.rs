// Generated macro for read (module)
macro_rules! Depcrateread {
() => {
// Module: crate
// Provides: {"read"}
// Dependencies: {}
# [cfg (feature = "std")] pub mod read { # ! [doc = " Reader-based hashing."] # ! [doc = ""] # ! [doc = " # Example"] # ! [doc = " ```rust"] # ! [doc = " use std::io::Cursor;"] # ! [doc = " use simd_adler32::read::adler32;"] # ! [doc = ""] # ! [doc = " let mut reader = Cursor::new(b\"Hello there\");"] # ! [doc = " let hash = adler32(&mut reader).unwrap();"] # ! [doc = ""] # ! [doc = " println!(\"{}\", hash) // 800813569"] # ! [doc = " ```"] use crate :: Adler32 ; use std :: io :: { Read , Result } ; # [doc = " Compute Adler-32 hash on reader until EOF."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use std::io::Cursor;"] # [doc = " use simd_adler32::read::adler32;"] # [doc = ""] # [doc = " let mut reader = Cursor::new(b\"Hello there\");"] # [doc = " let hash = adler32(&mut reader).unwrap();"] # [doc = ""] # [doc = " println!(\"{}\", hash) // 800813569"] # [doc = " ```"] pub fn adler32 < R : Read > (reader : & mut R) -> Result < u32 > { let mut hash = Adler32 :: new () ; let mut buf = [0 ; 4096] ; loop { match reader . read (& mut buf) { Ok (0) => return Ok (hash . finish ()) , Ok (n) => { hash . write (& buf [.. n]) ; } Err (err) => return Err (err) , } } } }
};
}
