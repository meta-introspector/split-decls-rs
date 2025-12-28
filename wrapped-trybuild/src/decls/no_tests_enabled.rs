macro_rules! no_tests_enabled {
    () => {
        pub (crate) fn no_tests_enabled () { term :: color (Yellow) ; println ! ("There are no trybuild tests enabled yet.") ; term :: reset () ; }
    };
}

no_tests_enabled!();