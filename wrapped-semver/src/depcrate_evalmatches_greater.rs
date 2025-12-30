// Generated macro for matches_greater (function)
macro_rules! Depcrate_evalmatches_greater {
() => {
// Module: crate::eval
// Provides: {"matches_greater"}
// Dependencies: {}
fn matches_greater (cmp : & Comparator , ver : & Version) -> bool { if ver . major != cmp . major { return ver . major > cmp . major ; } match cmp . minor { None => return false , Some (minor) => { if ver . minor != minor { return ver . minor > minor ; } } } match cmp . patch { None => return false , Some (patch) => { if ver . patch != patch { return ver . patch > patch ; } } } ver . pre > cmp . pre }
};
}
