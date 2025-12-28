macro_rules! deps {
    () => {
        Test!();
        Context!();
        Tera!();
    };
}

macro_rules! error_gives_source_on_tests {
    () => {
        deps!();
        # [test] fn error_gives_source_on_tests () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{% if a is undefined(1) %}-{% endif %}")]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; println ! ("{:?}" , result) ; let err = result . unwrap_err () ; let source = err . source () . unwrap () ; assert_eq ! (source . to_string () , "Test call \'undefined\' failed") ; let source2 = source . source () . unwrap () ; assert_eq ! (source2 . to_string () , "Tester `undefined` was called with some args but this test doesn\'t take args") ; }
    };
}

error_gives_source_on_tests!()