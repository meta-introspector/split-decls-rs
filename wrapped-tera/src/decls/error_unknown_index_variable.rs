macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! error_unknown_index_variable {
    () => {
        deps!();
        # [test] fn error_unknown_index_variable () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{{ arr[a] }}")]) . unwrap () ; let mut context = Context :: new () ; context . insert ("arr" , & [1 , 2 , 3]) ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Variable arr[a] can not be evaluated because: Variable `a` not found in context while rendering \'tpl\'") ; }
    };
}

error_unknown_index_variable!()