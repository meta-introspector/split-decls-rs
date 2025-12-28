macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! from_path {
    () => {
        deps!();
        # [test] fn from_path () -> Result < () , Error > { let bom : Bom = "fixtures/ascii.txt" . parse () ? ; assert_eq ! (bom , Bom :: Null) ; let bom : Bom = "fixtures/utf16-le.txt" . parse () ? ; assert_eq ! (bom , Bom :: Utf16Le) ; let bom : Bom = "fixtures/utf32-le.txt" . parse () ? ; assert_eq ! (bom , Bom :: Utf32Le) ; Ok (()) }
    };
}

from_path!()