macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! does_render_owned_for_loop_with_objects_string_keys {
    () => {
        deps!();
        # [test] fn does_render_owned_for_loop_with_objects_string_keys () { let mut context = Context :: new () ; let data = json ! ([{ "id" : 1 , "group" : "a" } , { "id" : 2 , "group" : "b" } , { "id" : 3 , "group" : "c" } , { "id" : 4 , "group" : "a" } , { "id" : 5 , "group" : "b" } , { "id" : 6 , "group" : "c" } , { "id" : 7 , "group" : "a" } , { "id" : 8 } , { "id" : 9 , "year" : null } ,]) ; context . insert ("something" , & data) ; let tpl = r#"{% for group, things in something | group_by(attribute="group") %}{{group}},{% endfor %}"# ; let expected = "a,b,c," ; assert_eq ! (render_template (tpl , & context) . unwrap () , expected) ; }
    };
}

does_render_owned_for_loop_with_objects_string_keys!()