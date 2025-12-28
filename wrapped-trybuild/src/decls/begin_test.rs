macro_rules! deps {
    () => {
        Test!();
        Expected!();
    };
}

macro_rules! begin_test {
    () => {
        deps!();
        pub (crate) fn begin_test (test : & Test , show_expected : bool) { let display_name = test . path . as_os_str () . to_string_lossy () ; print ! ("test ") ; term :: bold () ; print ! ("{}" , display_name) ; term :: reset () ; if show_expected { match test . expected { Expected :: Pass => print ! (" [should pass]") , Expected :: CompileFail => print ! (" [should fail to compile]") , } } print ! (" ... ") ; }
    };
}

begin_test!();