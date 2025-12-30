// Generated macro for parsing (module)
macro_rules! Depcrate_macparsing {
() => {
// Module: crate::mac
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub (crate) mod parsing { use crate :: error :: Result ; use crate :: mac :: { parse_delimiter , Macro } ; use crate :: parse :: { Parse , ParseStream } ; use crate :: path :: Path ; # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Macro { fn parse (input : ParseStream) -> Result < Self > { let tokens ; Ok (Macro { path : input . call (Path :: parse_mod_style) ? , bang_token : input . parse () ? , delimiter : { let (delimiter , content) = parse_delimiter (input) ? ; tokens = content ; delimiter } , tokens , }) } } }
};
}
