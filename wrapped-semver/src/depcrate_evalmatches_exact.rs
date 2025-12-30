// Generated macro for matches_exact (function)
macro_rules! Depcrate_evalmatches_exact {
() => {
// Module: crate::eval
// Provides: {"matches_exact"}
// Dependencies: {}
fn matches_exact (cmp : & Comparator , ver : & Version) -> bool { if ver . major != cmp . major { return false ; } if let Some (minor) = cmp . minor { if ver . minor != minor { return false ; } } if let Some (patch) = cmp . patch { if ver . patch != patch { return false ; } } ver . pre == cmp . pre }
};
}
