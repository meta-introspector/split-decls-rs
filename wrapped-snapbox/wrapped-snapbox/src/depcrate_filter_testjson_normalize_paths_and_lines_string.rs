// Generated macro for json_normalize_paths_and_lines_string (function)
macro_rules! Depcrate_filter_testjson_normalize_paths_and_lines_string {
() => {
// Module: crate::filter::test
// Provides: {"json_normalize_paths_and_lines_string"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_paths_and_lines_string () { let json = json ! ({ "name" : "John\\Doe\r\n" }) ; let data = Data :: json (json) ; let data = FilterPaths . filter (data) ; assert_eq ! (Data :: json (json ! ({ "name" : "John/Doe\r\n" })) , data) ; let data = FilterNewlines . filter (data) ; assert_eq ! (Data :: json (json ! ({ "name" : "John/Doe\n" })) , data) ; }
};
}
