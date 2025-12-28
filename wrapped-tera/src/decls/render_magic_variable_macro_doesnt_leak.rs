macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! render_magic_variable_macro_doesnt_leak {
    () => {
        deps!();
        # [test] fn render_magic_variable_macro_doesnt_leak () { let mut context = Context :: new () ; context . insert ("html" , & "<html>") ; context . insert ("num" , & 1) ; context . insert ("i" , & 10) ; let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello(arg=1) %}{{ __tera_context }}{% endmacro hello %}") , ("tpl" , "{% import \"macros\" as macros %}{{macros::hello()}}") ,]) . unwrap () ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap () , r#"{
  "arg": 1
}"# . to_owned ()) ; }
    };
}

render_magic_variable_macro_doesnt_leak!()