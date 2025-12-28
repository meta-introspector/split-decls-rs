macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! works_with_filter_section {
    () => {
        deps!();
        # [test] fn works_with_filter_section () { let mut context = Context :: new () ; context . insert ("d" , "d") ; let input = r#"{% filter upper %}  {{ "c" }}   d{% endfilter %}"# ; let res = Tera :: one_off (input , & context , true) . unwrap () ; assert_eq ! (res , "  C   D") ; }
    };
}

works_with_filter_section!()