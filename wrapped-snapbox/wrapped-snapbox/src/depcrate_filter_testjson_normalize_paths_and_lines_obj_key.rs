// Generated macro for json_normalize_paths_and_lines_obj_key (function)
macro_rules! Depcrate_filter_testjson_normalize_paths_and_lines_obj_key {
() => {
// Module: crate::filter::test
// Provides: {"json_normalize_paths_and_lines_obj_key"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_paths_and_lines_obj_key () { let json = json ! ({ "person" : { "John\\Doe\r\n" : "name" , "Jo\\hn\r\n" : "nickname" , } }) ; let data = Data :: json (json) ; let data = FilterPaths . filter (data) ; let assert = json ! ({ "person" : { "John/Doe\r\n" : "name" , "Jo/hn\r\n" : "nickname" , } }) ; assert_eq ! (Data :: json (assert) , data) ; let data = FilterNewlines . filter (data) ; let assert = json ! ({ "person" : { "John/Doe\n" : "name" , "Jo/hn\n" : "nickname" , } }) ; assert_eq ! (Data :: json (assert) , data) ; }
};
}
