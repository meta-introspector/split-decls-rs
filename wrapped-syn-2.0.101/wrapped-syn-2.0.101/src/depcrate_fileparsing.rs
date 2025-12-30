// Generated macro for parsing (module)
macro_rules! Depcrate_fileparsing {
() => {
// Module: crate::file
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub (crate) mod parsing { use crate :: attr :: Attribute ; use crate :: error :: Result ; use crate :: file :: File ; use crate :: parse :: { Parse , ParseStream } ; # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for File { fn parse (input : ParseStream) -> Result < Self > { Ok (File { shebang : None , attrs : input . call (Attribute :: parse_inner) ? , items : { let mut items = Vec :: new () ; while ! input . is_empty () { items . push (input . parse () ?) ; } items } , }) } } }
};
}
