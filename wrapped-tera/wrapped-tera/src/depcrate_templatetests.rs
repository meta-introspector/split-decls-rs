// Generated macro for tests (module)
macro_rules! Depcrate_templatetests {
() => {
// Module: crate::template
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Template ; # [test] fn can_parse_ok_template () { Template :: new ("hello" , None , "Hello {{ world }}.") . unwrap () ; } # [test] fn can_find_parent_template () { let tpl = Template :: new ("hello" , None , "{% extends \"base.html\" %}") . unwrap () ; assert_eq ! (tpl . parent . unwrap () , "base.html" . to_string ()) ; } # [test] fn can_find_blocks () { let tpl = Template :: new ("hello" , None , "{% extends \"base.html\" %}{% block hey %}{% endblock hey %}" ,) . unwrap () ; assert_eq ! (tpl . parent . unwrap () , "base.html" . to_string ()) ; assert ! (tpl . blocks . contains_key ("hey")) ; } # [test] fn can_find_nested_blocks () { let tpl = Template :: new ("hello" , None , "{% extends \"base.html\" %}{% block hey %}{% block extrahey %}{% endblock extrahey %}{% endblock hey %}" ,) . unwrap () ; assert_eq ! (tpl . parent . unwrap () , "base.html" . to_string ()) ; assert ! (tpl . blocks . contains_key ("hey")) ; assert ! (tpl . blocks . contains_key ("extrahey")) ; } # [test] fn can_find_macros () { let tpl = Template :: new ("hello" , None , "{% macro hey() %}{% endmacro hey %}") . unwrap () ; assert ! (tpl . macros . contains_key ("hey")) ; } # [test] fn can_find_imported_macros () { let tpl = Template :: new ("hello" , None , "{% import \"macros.html\" as macros %}") . unwrap () ; assert_eq ! (tpl . imported_macro_files , vec ! [("macros.html" . to_string () , "macros" . to_string ())]) ; } }
};
}
