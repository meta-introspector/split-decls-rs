macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! can_inherit_macro_import_from_parent {
    () => {
        deps!();
        # [test] fn can_inherit_macro_import_from_parent () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}HELLO{% endmacro hello %}") , ("parent" , "{% import \"macros\" as macros %}{% block bob %}parent{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}{{macros::hello()}}{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: default ()) ; assert_eq ! (result . unwrap () , "HELLO" . to_string ()) ; }
    };
}

can_inherit_macro_import_from_parent!()