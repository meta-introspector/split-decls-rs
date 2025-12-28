macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! render_magic_variable_isnt_escaped {
    () => {
        deps!();
        # [test] fn render_magic_variable_isnt_escaped () { let mut context = Context :: new () ; context . insert ("html" , & "<html>") ; let result = render_template ("{{ __tera_context }}" , & context) ; assert_eq ! (result . unwrap () , r#"{
  "html": "<html>"
}"# . to_owned ()) ; }
    };
}

render_magic_variable_isnt_escaped!();