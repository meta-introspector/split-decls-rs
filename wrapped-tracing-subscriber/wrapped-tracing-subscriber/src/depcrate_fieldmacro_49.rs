// Generated macro for macro_49 (macro)
macro_rules! Depcrate_fieldmacro_49 {
() => {
// Module: crate::field
// Provides: {"macro_49"}
// Dependencies: {}
feature ! { #! [feature = "std"] use std :: io ; # [doc = " Extension trait implemented by visitors to indicate that they write to an"] # [doc = " `io::Write` instance, and allow access to that writer."] pub trait VisitWrite : VisitOutput < Result < () , io :: Error >> { # [doc = " Returns the writer that this visitor writes to."] fn writer (& mut self) -> & mut dyn io :: Write ; } }
};
}
