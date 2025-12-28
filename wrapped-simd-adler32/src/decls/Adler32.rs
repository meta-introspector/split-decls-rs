macro_rules! Adler32 {
    () => {
        # [doc = " An adler32 hash generator type."] # [derive (Clone)] pub struct Adler32 { a : u16 , b : u16 , update : Adler32Imp , }
    };
}

Adler32!()