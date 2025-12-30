// Generated macro for race (function)
macro_rules! Depcraterace {
() => {
// Module: crate
// Provides: {"race"}
// Dependencies: {}
# [doc = " This function has been renamed to `select`; please see its documentation."] # [doc = " This function remains to maintain compatibility with the online versions"] # [doc = " of the book that use the name `race`."] pub async fn race < A , B , F1 , F2 > (f1 : F1 , f2 : F2) -> Either < A , B > where F1 : Future < Output = A > , F2 : Future < Output = B > , { select (f1 , f2) . await }
};
}
