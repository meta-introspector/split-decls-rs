// Generated macro for json_normalize_paths_and_lines_array_obj (function)
macro_rules! Depcrate_filter_testjson_normalize_paths_and_lines_array_obj {
() => {
// Module: crate::filter::test
// Provides: {"json_normalize_paths_and_lines_array_obj"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_paths_and_lines_array_obj () { let json = json ! ({ "people" : [{ "name" : "John\\Doe\r\n" , "nickname" : "Jo\\hn\r\n" , }] }) ; let data = Data :: json (json) ; let data = FilterPaths . filter (data) ; let paths = json ! ({ "people" : [{ "name" : "John/Doe\r\n" , "nickname" : "Jo/hn\r\n" , }] }) ; assert_eq ! (Data :: json (paths) , data) ; let data = FilterNewlines . filter (data) ; let new_lines = json ! ({ "people" : [{ "name" : "John/Doe\n" , "nickname" : "Jo/hn\n" , }] }) ; assert_eq ! (Data :: json (new_lines) , data) ; }
};
}
