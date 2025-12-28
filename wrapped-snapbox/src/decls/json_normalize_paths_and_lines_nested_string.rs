macro_rules! deps {
    () => {
        FilterPaths!();
        FilterNewlines!();
        Data!();
    };
}

macro_rules! json_normalize_paths_and_lines_nested_string {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_paths_and_lines_nested_string () { let json = json ! ({ "person" : { "name" : "John\\Doe\r\n" , "nickname" : "Jo\\hn\r\n" , } }) ; let data = Data :: json (json) ; let data = FilterPaths . filter (data) ; let assert = json ! ({ "person" : { "name" : "John/Doe\r\n" , "nickname" : "Jo/hn\r\n" , } }) ; assert_eq ! (Data :: json (assert) , data) ; let data = FilterNewlines . filter (data) ; let assert = json ! ({ "person" : { "name" : "John/Doe\n" , "nickname" : "Jo/hn\n" , } }) ; assert_eq ! (Data :: json (assert) , data) ; }
    };
}

json_normalize_paths_and_lines_nested_string!()