macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! error_location_basic {
    () => {
        deps!();
        # [test] fn error_location_basic () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{{ 1 + true }}")]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'tpl\'") ; }
    };
}

error_location_basic!();