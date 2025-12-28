macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! can_inherit_macro_import_from_grandparent {
    () => {
        deps!();
        # [test] fn can_inherit_macro_import_from_grandparent () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}HELLO{% endmacro hello %}") , ("grandparent" , "{% import \"macros\" as macros %}{% block bob %}grandparent{% endblock bob %}") , ("parent" , "{% extends \"grandparent\" %}{% import \"macros\" as macros2 %}{% block bob %}parent{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}{{macros::hello()}}-{{macros2::hello()}}{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: default ()) ; assert_eq ! (result . unwrap () , "HELLO-HELLO" . to_string ()) ; }
    };
}

can_inherit_macro_import_from_grandparent!();