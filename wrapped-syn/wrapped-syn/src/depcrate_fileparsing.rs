// Generated macro for parsing (module)
macro_rules! Depcrate_fileparsing {
() => {
// Module: crate::file
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub mod parsing { use super :: * ; use synom :: Synom ; impl Synom for File { named ! (parse -> Self , do_parse ! (attrs : many0 ! (call ! (Attribute :: parse_inner)) >> items : many0 ! (syn ! (Item)) >> (File { shebang : None , attrs : attrs , items : items , }))) ; fn description () -> Option < & 'static str > { Some ("crate") } } }
};
}
