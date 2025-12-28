macro_rules! deps {
    () => {
        Expected!();
    };
}

macro_rules! should_not_have_compiled {
    () => {
        deps!();
        pub (crate) fn should_not_have_compiled () { term :: bold_color (Red) ; println ! ("error") ; term :: color (Red) ; println ! ("Expected test case to fail to compile, but it succeeded.") ; term :: reset () ; println ! () ; }
    };
}

should_not_have_compiled!();