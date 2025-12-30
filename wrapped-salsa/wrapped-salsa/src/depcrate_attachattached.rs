// Generated macro for Attached (struct)
macro_rules! Depcrate_attachAttached {
() => {
// Module: crate::attach
// Provides: {"Attached"}
// Dependencies: {}
# [doc = " State that is specific to a single execution thread."] # [doc = ""] # [doc = " Internally, this type uses ref-cells."] # [doc = ""] # [doc = " **Note also that all mutations to the database handle (and hence"] # [doc = " to the local-state) must be undone during unwinding.**"] struct Attached { # [doc = " Pointer to the currently attached database."] database : Cell < Option < NonNull < dyn Database > > > , }
};
}
