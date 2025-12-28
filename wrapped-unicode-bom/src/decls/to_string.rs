macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! to_string {
    () => {
        deps!();
        # [test] fn to_string () { assert_eq ! (Bom :: Null . to_string () , Bom :: Null . as_ref ()) ; }
    };
}

to_string!();