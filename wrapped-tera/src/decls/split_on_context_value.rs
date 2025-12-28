macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! split_on_context_value {
    () => {
        deps!();
        # [test] fn split_on_context_value () { let mut tera = Tera :: default () ; tera . add_raw_template ("split.html" , r#"{{ body | split(pat="\n") }}"#) . unwrap () ; let mut context = Context :: new () ; context . insert ("body" , "multi\nple\nlines") ; let res = tera . render ("split.html" , & context) ; assert_eq ! (res . unwrap () , "[multi, ple, lines]") ; }
    };
}

split_on_context_value!();