// Generated macro for ModalError (trait)
macro_rules! Depcrate_errorModalError {
() => {
// Module: crate::error
// Provides: {"ModalError"}
// Dependencies: {}
# [doc = " Manipulate the how parsers respond to this error"] pub trait ModalError { # [doc = " Prevent backtracking, bubbling the error up to the top"] fn cut (self) -> Self ; # [doc = " Enable backtracking support"] fn backtrack (self) -> Self ; }
};
}
