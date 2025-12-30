// Generated macro for date_iso (function)
macro_rules! Depcratedate_iso {
() => {
// Module: crate
// Provides: {"date_iso"}
// Dependencies: {}
fn date_iso (sh : & Shell) -> anyhow :: Result < String > { let res = cmd ! (sh , "date -u +%Y-%m-%d") . read () ? ; Ok (res) }
};
}
