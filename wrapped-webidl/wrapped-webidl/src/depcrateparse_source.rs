// Generated macro for parse_source (function)
macro_rules! Depcrateparse_source {
() => {
// Module: crate
// Provides: {"parse_source"}
// Dependencies: {}
fn parse_source (source : & str) -> Result < Vec < weedle :: Definition < '_ > > > { match weedle :: Definitions :: parse (source) { Ok (("" , parsed)) => Ok (parsed) , Ok ((remaining , _)) | Err (weedle :: Err :: Error ((remaining , _))) | Err (weedle :: Err :: Failure ((remaining , _))) => { Err (WebIDLParseError (source . len () - remaining . len ()) . into ()) } Err (weedle :: Err :: Incomplete (needed)) => { Err (anyhow :: anyhow ! ("needed {needed:?} more bytes")) } } }
};
}
