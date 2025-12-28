macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! can_set_variables_in_included_templates {
    () => {
        deps!();
        # [test] fn can_set_variables_in_included_templates () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("world" , r#"{% set a = "world" %}{{a}}"#) , ("hello" , "<h1>Hello {% include \"world\" %}</h1>") ,]) . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) . unwrap () ; assert_eq ! (result , "<h1>Hello world</h1>" . to_owned ()) ; }
    };
}

can_set_variables_in_included_templates!();