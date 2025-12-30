// Generated macro for matches_tilde (function)
macro_rules! Depcrate_evalmatches_tilde {
() => {
// Module: crate::eval
// Provides: {"matches_tilde"}
// Dependencies: {}
fn matches_tilde (cmp : & Comparator , ver : & Version) -> bool { if ver . major != cmp . major { return false ; } if let Some (minor) = cmp . minor { if ver . minor != minor { return false ; } } if let Some (patch) = cmp . patch { if ver . patch != patch { return ver . patch > patch ; } } ver . pre >= cmp . pre }
};
}
