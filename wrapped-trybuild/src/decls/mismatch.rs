macro_rules! mismatch {
    () => {
        pub (crate) fn mismatch (expected : & str , actual : & str) { term :: bold_color (Red) ; println ! ("mismatch") ; term :: reset () ; println ! () ; let diff = if env :: var_os ("TERM") . map_or (true , | term | term == "dumb") { None } else { Diff :: compute (expected , actual) } ; term :: bold_color (Blue) ; println ! ("EXPECTED:") ; snippet_diff (Blue , expected , diff . as_ref ()) ; println ! () ; term :: bold_color (Red) ; println ! ("ACTUAL OUTPUT:") ; snippet_diff (Red , actual , diff . as_ref ()) ; print ! ("note: If the ") ; term :: color (Red) ; print ! ("actual output") ; term :: reset () ; println ! (" is the correct output you can bless it by rerunning") ; println ! ("      your test with the environment variable TRYBUILD=overwrite") ; println ! () ; }
    };
}

mismatch!()