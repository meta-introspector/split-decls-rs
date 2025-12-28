macro_rules! assert_err_msg {
    () => {
        fn assert_err_msg (input : & str , needles : & [& str]) { let res = parse (input) ; assert ! (res . is_err ()) ; let err = res . unwrap_err () ; let err_msg = err . to_string () ; println ! ("{}" , err_msg) ; println ! ("Looking for:") ; for needle in needles { println ! ("{}" , needle) ; assert ! (err_msg . contains (needle)) ; } }
    };
}

assert_err_msg!()