macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! make_sure_not_to_delete_whitespaces {
    () => {
        deps!();
        # [test] fn make_sure_not_to_delete_whitespaces () { let mut context = Context :: new () ; context . insert ("d" , "d") ; let input = r#"{% raw %}    yaml_test:     {% endraw %}"# ; let res = Tera :: one_off (input , & context , true) . unwrap () ; assert_eq ! (res , "    yaml_test:     ") ; }
    };
}

make_sure_not_to_delete_whitespaces!();