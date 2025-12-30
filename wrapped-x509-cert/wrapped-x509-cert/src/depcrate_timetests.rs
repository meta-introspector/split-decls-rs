// Generated macro for tests (module)
macro_rules! Depcrate_timetests {
() => {
// Module: crate::time
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn parse_time () { let time = Time :: from_str ("1970-01-01T00:00:00Z") . expect ("parse date from string") ; assert ! (matches ! (time , Time :: UtcTime (_))) ; assert_eq ! (alloc :: format ! ("{}" , time) , "1970-01-01T00:00:00Z") ; let time = Time :: from_str ("2020-01-01T00:00:00Z") . expect ("parse date from string") ; assert ! (matches ! (time , Time :: UtcTime (_))) ; assert_eq ! (alloc :: format ! ("{}" , time) , "2020-01-01T00:00:00Z") ; let time = Time :: from_str ("2049-12-31T23:59:59Z") . expect ("parse date from string") ; assert ! (matches ! (time , Time :: UtcTime (_))) ; assert_eq ! (alloc :: format ! ("{}" , time) , "2049-12-31T23:59:59Z") ; let time = Time :: from_str ("2050-01-01T00:00:00Z") . expect ("parse date from string") ; assert ! (matches ! (time , Time :: GeneralTime (_))) ; assert_eq ! (alloc :: format ! ("{}" , time) , "2050-01-01T00:00:00Z") ; } }
};
}
