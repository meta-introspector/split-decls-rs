macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! fail_output {
    () => {
        deps!();
        pub (crate) fn fail_output (level : Level , stdout : & str) { let color = match level { Fail => Red , Warn => Yellow , } ; if ! stdout . is_empty () { term :: bold_color (color) ; println ! ("STDOUT:") ; snippet (color , & normalize :: trim (stdout)) ; println ! () ; } }
    };
}

fail_output!();