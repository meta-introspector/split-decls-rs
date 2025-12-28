macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! filter_args_are_not_escaped {
    () => {
        deps!();
        # [test] fn filter_args_are_not_escaped () { let mut context = Context :: new () ; context . insert ("my_var" , & "hey") ; context . insert ("to" , & "&") ; let input = r#"{{ my_var | replace(from="h", to=to) }}"# ; assert_eq ! (render_template (input , & context) . unwrap () , "&amp;ey") ; }
    };
}

filter_args_are_not_escaped!();