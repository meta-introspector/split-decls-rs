// Generated macro for is_max_date (function)
macro_rules! Depcrateis_max_date {
() => {
// Module: crate
// Provides: {"is_max_date"}
// Dependencies: {}
# [doc = " Checks that the running or installed `rustc` was released **on or before**"] # [doc = " some date."] # [doc = ""] # [doc = " The format of `max_date` must be YYYY-MM-DD. For instance: `2016-12-20` or"] # [doc = " `2017-01-09`."] # [doc = ""] # [doc = " If the date cannot be retrieved or parsed, or if `max_date` could not be"] # [doc = " parsed, returns `None`. Otherwise returns `true` if the installed `rustc`"] # [doc = " was release on or before `max_date` and `false` otherwise."] pub fn is_max_date (max_date : & str) -> Option < bool > { match (Date :: read () , Date :: parse (max_date)) { (Some (rustc_date) , Some (max_date)) => Some (rustc_date <= max_date) , _ => None , } }
};
}
