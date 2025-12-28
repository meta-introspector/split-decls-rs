macro_rules! deps {
    () => {
        FilterNewlines!();
        FilterPaths!();
        Data!();
    };
}

macro_rules! json_normalize_paths_and_lines_string {
    () => {
        deps!();
        # [test] # [cfg (feature = "json")] fn json_normalize_paths_and_lines_string () { let json = json ! ({ "name" : "John\\Doe\r\n" }) ; let data = Data :: json (json) ; let data = FilterPaths . filter (data) ; assert_eq ! (Data :: json (json ! ({ "name" : "John/Doe\r\n" })) , data) ; let data = FilterNewlines . filter (data) ; assert_eq ! (Data :: json (json ! ({ "name" : "John/Doe\n" })) , data) ; }
    };
}

json_normalize_paths_and_lines_string!()