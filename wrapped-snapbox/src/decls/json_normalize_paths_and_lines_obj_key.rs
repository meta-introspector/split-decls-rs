macro_rules! deps {
    () => {
        FilterPaths!();
        Data!();
        FilterNewlines!();
    };
}

macro_rules! json_normalize_paths_and_lines_obj_key {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_paths_and_lines_obj_key () { let json = json ! ({ "person" : { "John\\Doe\r\n" : "name" , "Jo\\hn\r\n" : "nickname" , } }) ; let data = Data :: json (json) ; let data = FilterPaths . filter (data) ; let assert = json ! ({ "person" : { "John/Doe\r\n" : "name" , "Jo/hn\r\n" : "nickname" , } }) ; assert_eq ! (Data :: json (assert) , data) ; let data = FilterNewlines . filter (data) ; let assert = json ! ({ "person" : { "John/Doe\n" : "name" , "Jo/hn\n" : "nickname" , } }) ; assert_eq ! (Data :: json (assert) , data) ; }
    };
}

json_normalize_paths_and_lines_obj_key!();