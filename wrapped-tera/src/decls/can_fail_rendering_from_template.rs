macro_rules! deps {
    () => {
        Function!();
        Context!();
        Error!();
    };
}

macro_rules! can_fail_rendering_from_template {
    () => {
        deps!();
        # [test] fn can_fail_rendering_from_template () { let mut context = Context :: new () ; context . insert ("title" , "hello") ; let res = render_template (r#"{{ throw(message="Error: " ~ title ~ " did not include a summary") }}"# , & context ,) ; let err = res . expect_err ("This should always fail to render") ; let source = err . source () . expect ("Must have a source") ; assert_eq ! (source . to_string () , "Function call 'throw' failed") ; let source = source . source () . expect ("Should have a nested error") ; assert_eq ! (source . to_string () , "Error: hello did not include a summary") ; }
    };
}

can_fail_rendering_from_template!();