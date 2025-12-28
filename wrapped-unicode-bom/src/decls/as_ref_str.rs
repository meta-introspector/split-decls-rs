macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! as_ref_str {
    () => {
        deps!();
        # [test] fn as_ref_str () { assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Null) , "[not set]") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Bocu1) , "BOCU-1") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Gb18030) , "GB 18030") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Scsu) , "SCSU") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: UtfEbcdic) , "UTF-EBCDIC") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Utf1) , "UTF-1") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Utf7) , "UTF-7") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Utf8) , "UTF-8") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Utf16Be) , "UTF-16 (big-endian)") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Utf16Le) , "UTF-16 (little-endian)") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Utf32Be) , "UTF-32 (big-endian)") ; assert_eq ! (AsRef ::< str >:: as_ref (& Bom :: Utf32Le) , "UTF-32 (little-endian)") ; }
    };
}

as_ref_str!();