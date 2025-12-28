macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! test_fail {
    () => {
        deps!();
        pub (crate) fn test_fail (err : Error) { if err . already_printed () { return ; } term :: bold_color (Red) ; println ! ("error") ; term :: color (Red) ; println ! ("{}" , err) ; term :: reset () ; println ! () ; }
    };
}

test_fail!()