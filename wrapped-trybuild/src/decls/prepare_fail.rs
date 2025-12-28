macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! prepare_fail {
    () => {
        deps!();
        pub (crate) fn prepare_fail (err : Error) { if err . already_printed () { return ; } term :: bold_color (Red) ; print ! ("ERROR") ; term :: reset () ; println ! (": {}" , err) ; println ! () ; }
    };
}

prepare_fail!()