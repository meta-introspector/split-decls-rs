macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Bom { # [doc = " Returns the size in bytes of the BOM."] pub fn len (& self) -> usize { match * self { Bom :: Null => 0 , Bom :: Bocu1 => 3 , Bom :: Gb18030 => 4 , Bom :: Scsu => 3 , Bom :: UtfEbcdic => 4 , Bom :: Utf1 => 3 , Bom :: Utf7 => 4 , Bom :: Utf8 => 3 , Bom :: Utf16Be => 2 , Bom :: Utf16Le => 2 , Bom :: Utf32Be => 4 , Bom :: Utf32Le => 4 , } } }
    };
}

impl_2!()