// Generated macro for errors_with_inheritance_in_included_template (function)
macro_rules! Depcrate_renderer_tests_errorserrors_with_inheritance_in_included_template {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"errors_with_inheritance_in_included_template"}
// Dependencies: {}
# [test] fn errors_with_inheritance_in_included_template () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("base" , "Base - {% include \"child\" %}") , ("parent" , "{% block title %}Parent{% endblock %}") , ("child" , "{% extends \"parent\" %}{% block title %}{{ super() }} - Child{% endblock %}") ,]) . unwrap () ; let result = tera . render ("base" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Inheritance in included templates is currently not supported: extended `parent`") ; }
};
}
