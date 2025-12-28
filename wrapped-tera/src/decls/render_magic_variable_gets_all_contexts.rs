macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! render_magic_variable_gets_all_contexts {
    () => {
        deps!();
        # [test] fn render_magic_variable_gets_all_contexts () { let mut context = Context :: new () ; context . insert ("html" , & "<html>") ; context . insert ("num" , & 1) ; context . insert ("i" , & 10) ; let result = render_template ("{% set some_val = 1 %}{% for i in range(start=0, end=1) %}{% set for_val = i %}{{ __tera_context }}{% endfor %}" , & context) ; assert_eq ! (result . unwrap () , r#"{
  "for_val": 0,
  "html": "<html>",
  "i": 0,
  "num": 1,
  "some_val": 1
}"# . to_owned ()) ; }
    };
}

render_magic_variable_gets_all_contexts!();