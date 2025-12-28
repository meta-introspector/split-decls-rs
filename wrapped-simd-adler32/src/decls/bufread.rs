macro_rules! deps {
    () => {
        Adler32!();
    };
}

macro_rules! bufread {
    () => {
        deps!();
        # [cfg (feature = "std")] pub mod bufread { # ! [doc = " BufRead-based hashing."] # ! [doc = ""] # ! [doc = " Separate `BufRead` trait implemented to allow for custom buffer size optimization."] # ! [doc = ""] # ! [doc = " # Example"] # ! [doc = " ```rust"] # ! [doc = " use std::io::{Cursor, BufReader};"] # ! [doc = " use simd_adler32::bufread::adler32;"] # ! [doc = ""] # ! [doc = " let mut reader = Cursor::new(b\"Hello there\");"] # ! [doc = " let mut reader = BufReader::new(reader);"] # ! [doc = " let hash = adler32(&mut reader).unwrap();"] # ! [doc = ""] # ! [doc = " println!(\"{}\", hash) // 800813569"] # ! [doc = " ```"] use crate :: Adler32 ; use std :: io :: { BufRead , ErrorKind , Result } ; # [doc = " Compute Adler-32 hash on buf reader until EOF."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use std::io::{Cursor, BufReader};"] # [doc = " use simd_adler32::bufread::adler32;"] # [doc = ""] # [doc = " let mut reader = Cursor::new(b\"Hello there\");"] # [doc = " let mut reader = BufReader::new(reader);"] # [doc = " let hash = adler32(&mut reader).unwrap();"] # [doc = ""] # [doc = " println!(\"{}\", hash) // 800813569"] # [doc = " ```"] pub fn adler32 < R : BufRead > (reader : & mut R) -> Result < u32 > { let mut hash = Adler32 :: new () ; loop { let consumed = match reader . fill_buf () { Ok (buf) => { if buf . is_empty () { return Ok (hash . finish ()) ; } hash . write (buf) ; buf . len () } Err (err) => match err . kind () { ErrorKind :: Interrupted => continue , ErrorKind :: UnexpectedEof => return Ok (hash . finish ()) , _ => return Err (err) , } , } ; reader . consume (consumed) ; } } }
    };
}

bufread!()