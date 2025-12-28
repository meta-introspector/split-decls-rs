macro_rules! deps {
    () => {
        FilterNewlines!();
        FilterPaths!();
        Data!();
    };
}

macro_rules! json_normalize_paths_and_lines_array {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_paths_and_lines_array () { let json = json ! ({ "people" : ["John\\Doe\r\n" , "Jo\\hn\r\n"] }) ; let data = Data :: json (json) ; let data = FilterPaths . filter (data) ; let paths = json ! ({ "people" : ["John/Doe\r\n" , "Jo/hn\r\n"] }) ; assert_eq ! (Data :: json (paths) , data) ; let data = FilterNewlines . filter (data) ; let new_lines = json ! ({ "people" : ["John/Doe\n" , "Jo/hn\n"] }) ; assert_eq ! (Data :: json (new_lines) , data) ; }
    };
}

json_normalize_paths_and_lines_array!();