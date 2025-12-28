macro_rules! deps {
    () => {
        TinyStr8!();
        TinyStr4!();
    };
}

macro_rules! test_size {
    () => {
        deps!();
        # [test] fn test_size () { assert_eq ! (core :: mem :: size_of ::< TinyStr4 > () , core :: mem :: size_of ::< Option < TinyStr4 >> ()) ; assert_eq ! (core :: mem :: size_of ::< TinyStr8 > () , core :: mem :: size_of ::< Option < TinyStr8 >> ()) ; }
    };
}

test_size!()