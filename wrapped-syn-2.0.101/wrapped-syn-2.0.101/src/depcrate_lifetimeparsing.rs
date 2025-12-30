// Generated macro for parsing (module)
macro_rules! Depcrate_lifetimeparsing {
() => {
// Module: crate::lifetime
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub (crate) mod parsing { use crate :: error :: Result ; use crate :: lifetime :: Lifetime ; use crate :: parse :: { Parse , ParseStream } ; # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Lifetime { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | { cursor . lifetime () . ok_or_else (| | cursor . error ("expected lifetime")) }) } } }
};
}
