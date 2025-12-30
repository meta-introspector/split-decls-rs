// Generated macro for Init (struct)
macro_rules! Depcrate_move_pathsInit {
() => {
// Module: crate::move_paths
// Provides: {"Init"}
// Dependencies: {}
# [doc = " `Init` represents a point in a program that initializes some L-value;"] # [derive (Copy , Clone)] pub struct Init { # [doc = " path being initialized"] pub path : MovePathIndex , # [doc = " location of initialization"] pub location : InitLocation , # [doc = " Extra information about this initialization"] pub kind : InitKind , }
};
}
