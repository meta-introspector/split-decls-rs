macro_rules! deps {
    () => {
        EncodedPoint!();
        Error!();
        Result!();
        ModulusSize!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [doc = " Decode a SEC1-encoded point from hexadecimal."] # [doc = ""] # [doc = " Upper and lower case hexadecimal are both accepted, however mixed case is"] # [doc = " rejected."] impl < Size > str :: FromStr for EncodedPoint < Size > where Size : ModulusSize , { type Err = Error ; fn from_str (hex : & str) -> Result < Self > { let mut buf = Array :: < u8 , Size :: UncompressedPointSize > :: default () ; base16ct :: mixed :: decode (hex , & mut buf) . map_err (| _ | Error :: PointEncoding) . and_then (Self :: from_bytes) } }
    };
}

impl_20!()