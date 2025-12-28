macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! error_location_base_template {
    () => {
        deps!();
        # [test] fn error_location_base_template () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , "Hello {{ greeting + 1}} {% block bob %}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}Hey{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'child\' (error happened in 'parent').") ; }
    };
}

error_location_base_template!()