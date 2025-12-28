macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! len {
    () => {
        deps!();
        # [test] fn len () { assert_eq ! (Bom :: Null . len () , 0) ; assert_eq ! (Bom :: Bocu1 . len () , 3) ; assert_eq ! (Bom :: Gb18030 . len () , 4) ; assert_eq ! (Bom :: Scsu . len () , 3) ; assert_eq ! (Bom :: UtfEbcdic . len () , 4) ; assert_eq ! (Bom :: Utf1 . len () , 3) ; assert_eq ! (Bom :: Utf7 . len () , 4) ; assert_eq ! (Bom :: Utf8 . len () , 3) ; assert_eq ! (Bom :: Utf16Be . len () , 2) ; assert_eq ! (Bom :: Utf16Le . len () , 2) ; assert_eq ! (Bom :: Utf32Be . len () , 4) ; assert_eq ! (Bom :: Utf32Le . len () , 4) ; }
    };
}

len!()