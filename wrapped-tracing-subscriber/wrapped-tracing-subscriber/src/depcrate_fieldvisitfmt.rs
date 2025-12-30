// Generated macro for VisitFmt (trait)
macro_rules! Depcrate_fieldVisitFmt {
() => {
// Module: crate::field
// Provides: {"VisitFmt"}
// Dependencies: {}
# [doc = " Extension trait implemented by visitors to indicate that they write to a"] # [doc = " `fmt::Write` instance, and allow access to that writer."] pub trait VisitFmt : VisitOutput < fmt :: Result > { # [doc = " Returns the formatter that this visitor writes to."] fn writer (& mut self) -> & mut dyn fmt :: Write ; }
};
}
