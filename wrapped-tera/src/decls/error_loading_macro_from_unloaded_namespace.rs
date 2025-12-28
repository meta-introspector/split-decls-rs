macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! error_loading_macro_from_unloaded_namespace {
    () => {
        deps!();
        # [test] fn error_loading_macro_from_unloaded_namespace () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 + true }}{% endmacro hello %}") , ("tpl" , "{% import \"macros\" as macros %}{{ macro::hello() }}") ,]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; println ! ("{:#?}" , result) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Macro namespace `macro` was not found in template `tpl`. Have you maybe forgotten to import it, or misspelled it?") ; }
    };
}

error_loading_macro_from_unloaded_namespace!();