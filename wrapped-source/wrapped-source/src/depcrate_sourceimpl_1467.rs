// Generated macro for impl_1467 (impl)
macro_rules! Depcrate_sourceimpl_1467 {
() => {
// Module: crate::source
// Provides: {"impl_1467"}
// Dependencies: {}
impl TzdbCache { pub (crate) fn parsed (& self) -> Result < & Tzdb , DataError > { self . transitions . get_or_init (| | { fn parse (lines : Vec < String >) -> parse_zoneinfo :: table :: Table { use parse_zoneinfo :: line :: Line ; use parse_zoneinfo :: table :: TableBuilder ; let mut table = TableBuilder :: new () ; for line in lines { match Line :: new (& line) . unwrap () { Line :: Zone (zone) => table . add_zone_line (zone) . unwrap () , Line :: Continuation (cont) => table . add_continuation_line (cont) . unwrap () , Line :: Rule (rule) => table . add_rule_line (rule) . unwrap () , Line :: Link (link) => table . add_link_line (link) . unwrap () , Line :: Space => { } } } table . build () } Ok (Tzdb { main : parse (["africa" , "antarctica" , "asia" , "australasia" , "europe" , "northamerica" , "southamerica" , "etcetera" , "factory" , "backward" ,] . into_iter () . try_fold (Vec :: new () , | mut lines , file | { lines . extend (self . root . read_to_string (file) ? . lines () . map (ToOwned :: to_owned) ,) ; Ok :: < _ , DataError > (lines) }) ? ,) , rearguard : self . root . file_exists ("rearguard.zi") ? . then (| | { parse (self . root . read_to_string ("rearguard.zi") . unwrap () . lines () . map (ToOwned :: to_owned) . collect () ,) }) , vanguard : self . root . file_exists ("vanguard.zi") ? . then (| | { parse (self . root . read_to_string ("vanguard.zi") . unwrap () . lines () . map (ToOwned :: to_owned) . collect () ,) }) , }) }) . as_ref () . map_err (| & e | e) } }
};
}
