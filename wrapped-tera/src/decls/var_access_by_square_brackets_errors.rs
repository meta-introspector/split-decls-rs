macro_rules! deps {
    () => {
        Context!();
        Test!();
        Tera!();
    };
}

macro_rules! var_access_by_square_brackets_errors {
    () => {
        deps!();
        # [test] fn var_access_by_square_brackets_errors () { let mut context = Context :: new () ; context . insert ("var" , & Test { a : "hi" . into () , b : "there" . into () , c : vec ! [] }) ; let t = Tera :: one_off ("{{var[csd]}}" , & context , true) ; assert ! (t . is_err () , "Access of csd should be impossible") ; }
    };
}

var_access_by_square_brackets_errors!();