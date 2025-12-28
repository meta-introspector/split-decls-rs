macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! can_get_value_if_key_contains_period {
    () => {
        deps!();
        # [test] fn can_get_value_if_key_contains_period () { let mut context = Context :: new () ; context . insert ("name" , "Mt. Robson Provincial Park") ; let mut map = HashMap :: new () ; map . insert ("Mt. Robson Provincial Park" . to_string () , "hello" . to_string ()) ; context . insert ("tag_info" , & map) ; let res = Tera :: one_off (r#"{{ tag_info[name] }}"# , & context , true) ; assert ! (res . is_ok ()) ; let res = res . unwrap () ; assert_eq ! (res , "hello") ; }
    };
}

can_get_value_if_key_contains_period!();