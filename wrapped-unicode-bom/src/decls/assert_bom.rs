macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! assert_bom {
    () => {
        deps!();
        fn assert_bom (bytes : & [u8] , expected : Bom) { assert_eq ! (Bom :: from (bytes) , expected) ; }
    };
}

assert_bom!()