// Generated macro for ungroup (function)
macro_rules! Depcrate_internalsungroup {
() => {
// Module: crate::internals
// Provides: {"ungroup"}
// Dependencies: {}
pub fn ungroup (mut ty : & Type) -> & Type { while let Type :: Group (group) = ty { ty = & group . elem ; } ty }
};
}
