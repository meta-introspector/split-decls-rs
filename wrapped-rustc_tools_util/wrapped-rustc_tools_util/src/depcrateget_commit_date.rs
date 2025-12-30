// Generated macro for get_commit_date (function)
macro_rules! Depcrateget_commit_date {
() => {
// Module: crate
// Provides: {"get_commit_date"}
// Dependencies: {}
# [must_use] pub fn get_commit_date () -> Option < String > { get_output ("git" , & ["log" , "-1" , "--date=short" , "--pretty=format:%cd"]) }
};
}
