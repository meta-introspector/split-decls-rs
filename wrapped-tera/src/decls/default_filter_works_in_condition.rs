macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! default_filter_works_in_condition {
    () => {
        deps!();
        # [test] fn default_filter_works_in_condition () { let mut tera = Tera :: default () ; tera . add_raw_template ("test.html" , r#"{% if frobnicate|default(value=True) %}here{% endif %}"#) . unwrap () ; let res = tera . render ("test.html" , & Context :: new ()) ; assert_eq ! (res . unwrap () , "here") ; }
    };
}

default_filter_works_in_condition!()