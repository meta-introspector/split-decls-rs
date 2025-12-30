// Generated macro for pre_is_compatible (function)
macro_rules! Depcrate_evalpre_is_compatible {
() => {
// Module: crate::eval
// Provides: {"pre_is_compatible"}
// Dependencies: {}
fn pre_is_compatible (cmp : & Comparator , ver : & Version) -> bool { cmp . major == ver . major && cmp . minor == Some (ver . minor) && cmp . patch == Some (ver . patch) && ! cmp . pre . is_empty () }
};
}
