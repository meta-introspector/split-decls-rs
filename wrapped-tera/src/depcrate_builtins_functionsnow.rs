// Generated macro for now (function)
macro_rules! Depcrate_builtins_functionsnow {
() => {
// Module: crate::builtins::functions
// Provides: {"now"}
// Dependencies: {}
# [cfg (feature = "builtins")] pub fn now (args : & HashMap < String , Value >) -> Result < Value > { let utc = match args . get ("utc") { Some (val) => match from_value :: < bool > (val . clone ()) { Ok (v) => v , Err (_) => { return Err (Error :: msg (format ! ("Function `now` received utc={} but `utc` can only be a boolean" , val))) ; } } , None => false , } ; let timestamp = match args . get ("timestamp") { Some (val) => match from_value :: < bool > (val . clone ()) { Ok (v) => v , Err (_) => { return Err (Error :: msg (format ! ("Function `now` received timestamp={} but `timestamp` can only be a boolean" , val))) ; } } , None => false , } ; if utc { let datetime = Utc :: now () ; if timestamp { return Ok (to_value (datetime . timestamp ()) . unwrap ()) ; } Ok (to_value (datetime . to_rfc3339 ()) . unwrap ()) } else { let datetime = Local :: now () ; if timestamp { return Ok (to_value (datetime . timestamp ()) . unwrap ()) ; } Ok (to_value (datetime . to_rfc3339 ()) . unwrap ()) } }
};
}
