macro_rules! deps {
    () => {
        Error!();
        Tag!();
        Result!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Tag { # [doc = " Parse a tag value from a byte"] pub fn from_u8 (byte : u8) -> Result < Self > { match byte { 0 => Ok (Tag :: Identity) , 2 => Ok (Tag :: CompressedEvenY) , 3 => Ok (Tag :: CompressedOddY) , 4 => Ok (Tag :: Uncompressed) , 5 => Ok (Tag :: Compact) , _ => Err (Error :: PointEncoding) , } } # [doc = " Is this point compact?"] pub fn is_compact (self) -> bool { matches ! (self , Tag :: Compact) } # [doc = " Is this point compressed?"] pub fn is_compressed (self) -> bool { matches ! (self , Tag :: CompressedEvenY | Tag :: CompressedOddY) } # [doc = " Is this point the identity point?"] pub fn is_identity (self) -> bool { self == Tag :: Identity } # [doc = " Compute the expected total message length for a message prefixed"] # [doc = " with this tag (including the tag byte), given the field element size"] # [doc = " (in bytes) for a particular elliptic curve."] pub fn message_len (self , field_element_size : usize) -> usize { 1 + match self { Tag :: Identity => 0 , Tag :: CompressedEvenY | Tag :: CompressedOddY => field_element_size , Tag :: Uncompressed => field_element_size * 2 , Tag :: Compact => field_element_size , } } # [doc = " Compress the given y-coordinate, returning a `Tag::Compressed*` value"] fn compress_y (y : & [u8]) -> Self { if y . as_ref () . last () . expect ("empty y-coordinate") & 1 == 1 { Tag :: CompressedOddY } else { Tag :: CompressedEvenY } } }
    };
}

impl_26!()