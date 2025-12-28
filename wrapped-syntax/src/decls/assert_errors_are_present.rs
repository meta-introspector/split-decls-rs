macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! assert_errors_are_present {
    () => {
        deps!();
        fn assert_errors_are_present (errors : & [SyntaxError] , path : & Path) { assert ! (! errors . is_empty () , "There should be errors in the file {:?}" , path . display ()) ; }
    };
}

assert_errors_are_present!()