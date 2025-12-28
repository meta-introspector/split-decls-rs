macro_rules! deps {
    () => {
        Tera!();
        Template!();
    };
}

macro_rules! error_when_missing_macro_templates {
    () => {
        deps!();
        # [test] fn error_when_missing_macro_templates () { let mut tera = Tera :: default () ; let result = tera . add_raw_templates (vec ! [("parent" , "{% import \"macros\" as macros %}{{ macros::hello() }}{% block bob %}{% endblock bob %}" ,)]) ; assert_eq ! (result . unwrap_err () . to_string () , "Template `parent` loads macros from `macros` which isn\'t present in Tera") ; }
    };
}

error_when_missing_macro_templates!()