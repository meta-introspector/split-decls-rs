macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! error_invalid_type_index_variable {
    () => {
        deps!();
        # [test] fn error_invalid_type_index_variable () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{{ arr[a] }}")]) . unwrap () ; let mut context = Context :: new () ; context . insert ("arr" , & [1 , 2 , 3]) ; context . insert ("a" , & true) ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Only variables evaluating to String or Number can be used as index (`a` of `arr[a]`)") ; }
    };
}

error_invalid_type_index_variable!();