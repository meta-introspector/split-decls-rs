macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! does_render_owned_for_loop_with_objects {
    () => {
        deps!();
        # [test] fn does_render_owned_for_loop_with_objects () { let mut context = Context :: new () ; let data = json ! ([{ "id" : 1 , "year" : 2015 } , { "id" : 2 , "year" : 2015 } , { "id" : 3 , "year" : 2016 } , { "id" : 4 , "year" : 2017 } , { "id" : 5 , "year" : 2017 } , { "id" : 6 , "year" : 2017 } , { "id" : 7 , "year" : 2018 } , { "id" : 8 } , { "id" : 9 , "year" : null } ,]) ; context . insert ("something" , & data) ; let tpl = r#"{% for year, things in something | group_by(attribute="year") %}{{year}},{% endfor %}"# ; let expected = "2015,2016,2017,2018," ; assert_eq ! (render_template (tpl , & context) . unwrap () , expected) ; }
    };
}

does_render_owned_for_loop_with_objects!()