macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! from_file {
    () => {
        deps!();
        # [test] fn from_file () { let mut file = File :: open ("fixtures/ascii.txt") . unwrap () ; assert_eq ! (Bom :: from (& mut file) , Bom :: Null) ; let mut file = File :: open ("fixtures/utf16-le.txt") . unwrap () ; assert_eq ! (Bom :: from (& mut file) , Bom :: Utf16Le) ; let mut file = File :: open ("fixtures/utf32-le.txt") . unwrap () ; assert_eq ! (Bom :: from (& mut file) , Bom :: Utf32Le) ; }
    };
}

from_file!();