// Generated macro for json_schema (macro)
macro_rules! Depcrate_macrosjson_schema {
() => {
// Module: crate::macros
// Provides: {"json_schema"}
// Dependencies: {}
# [doc = " Construct a [`Schema`](crate::Schema) from a JSON literal. This can either be a JSON object, or"] # [doc = " a boolean (`true` or `false`)."] # [doc = ""] # [doc = " You can interpolate variables or expressions into a JSON object using the same rules as the"] # [doc = " [`serde_json::json`] macro."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use schemars::{Schema, json_schema};"] # [doc = ""] # [doc = " let desc = \"A helpful description.\";"] # [doc = " let my_schema: Schema = json_schema!({"] # [doc = "     \"description\": desc,"] # [doc = "     \"type\": [\"object\", \"null\"]"] # [doc = " });"] # [doc = " ```"] # [macro_export] macro_rules ! json_schema { ({ $ ($ json_object : tt) * }) => { <$ crate :: Schema as :: core :: convert :: TryFrom < _ >>:: try_from ($ crate :: _private :: serde_json :: json ! ({ $ ($ json_object) * })) . unwrap () } ; (true) => { $ crate :: Schema :: from (true) } ; (false) => { $ crate :: Schema :: from (false) } ; }
};
}
