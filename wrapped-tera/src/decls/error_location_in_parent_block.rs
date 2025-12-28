macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! error_location_in_parent_block {
    () => {
        deps!();
        # [test] fn error_location_in_parent_block () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , "Hello {{ greeting }} {% block bob %}{{ 1 + true }}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}{{ super() }}Hey{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'child\' (error happened in 'parent').") ; }
    };
}

error_location_in_parent_block!()