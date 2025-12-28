macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! error_out_of_range_index {
    () => {
        deps!();
        # [test] fn error_out_of_range_index () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{{ arr[10] }}")]) . unwrap () ; let mut context = Context :: new () ; context . insert ("arr" , & [1 , 2 , 3]) ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Variable `arr[10]` not found in context while rendering \'tpl\': the evaluated version was `arr.10`. Maybe the index is out of bounds?") ; }
    };
}

error_out_of_range_index!();