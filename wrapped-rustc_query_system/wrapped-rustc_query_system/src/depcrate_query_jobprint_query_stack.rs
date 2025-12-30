// Generated macro for print_query_stack (function)
macro_rules! Depcrate_query_jobprint_query_stack {
() => {
// Module: crate::query::job
// Provides: {"print_query_stack"}
// Dependencies: {}
pub fn print_query_stack < Qcx : QueryContext > (qcx : Qcx , mut current_query : Option < QueryJobId > , dcx : DiagCtxtHandle < '_ > , limit_frames : Option < usize > , mut file : Option < std :: fs :: File > ,) -> usize { let mut count_printed = 0 ; let mut count_total = 0 ; let query_map = match qcx . collect_active_jobs () { Ok (query_map) => query_map , Err (query_map) => query_map , } ; if let Some (ref mut file) = file { let _ = writeln ! (file , "\n\nquery stack during panic:") ; } while let Some (query) = current_query { let Some (query_info) = query_map . get (& query) else { break ; } ; let query_extra = qcx . lift_query_info (& query_info . query . info) ; if Some (count_printed) < limit_frames || limit_frames . is_none () { # [allow (rustc :: diagnostic_outside_of_impl)] # [allow (rustc :: untranslatable_diagnostic)] dcx . struct_failure_note (format ! ("#{} [{:?}] {}" , count_printed , query_info . query . dep_kind , query_extra . description)) . with_span (query_info . job . span) . emit () ; count_printed += 1 ; } if let Some (ref mut file) = file { let _ = writeln ! (file , "#{} [{}] {}" , count_total , qcx . dep_context () . dep_kind_info (query_info . query . dep_kind) . name , query_extra . description) ; } current_query = query_info . job . parent ; count_total += 1 ; } if let Some (ref mut file) = file { let _ = writeln ! (file , "end of query stack") ; } count_total }
};
}
