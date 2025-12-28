macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! from_as_ref_arr {
    () => {
        deps!();
        # [test] fn from_as_ref_arr () { assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Null) , Bom :: Null) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Bocu1) , Bom :: Bocu1) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Gb18030) , Bom :: Gb18030) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Scsu) , Bom :: Scsu) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: UtfEbcdic) , Bom :: UtfEbcdic) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Utf1) , Bom :: Utf1) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Utf8) , Bom :: Utf8) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Utf16Be) , Bom :: Utf16Be) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Utf16Le) , Bom :: Utf16Le) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Utf32Be) , Bom :: Utf32Be) ; assert_bom (AsRef :: < [u8] > :: as_ref (& Bom :: Utf32Le) , Bom :: Utf32Le) ; }
    };
}

from_as_ref_arr!()