// Generated macro for find_attr_val (function)
macro_rules! Depcrate_featuresfind_attr_val {
() => {
// Module: crate::features
// Provides: {"find_attr_val"}
// Dependencies: {}
fn find_attr_val < 'a > (line : & 'a str , attr : & str) -> Option < & 'a str > { let r = match attr { "issue" => static_regex ! (r#"issue\s*=\s*"([^"]*)""#) , "feature" => static_regex ! (r#"feature\s*=\s*"([^"]*)""#) , "since" => static_regex ! (r#"since\s*=\s*"([^"]*)""#) , _ => unimplemented ! ("{attr} not handled") , } ; r . captures (line) . and_then (| c | c . get (1)) . map (| m | m . as_str ()) }
};
}
