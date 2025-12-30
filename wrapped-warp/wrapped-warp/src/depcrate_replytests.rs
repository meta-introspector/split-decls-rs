// Generated macro for tests (module)
macro_rules! Depcrate_replytests {
() => {
// Module: crate::reply
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: collections :: HashMap ; use super :: * ; # [test] fn json_serde_error () { let mut map = HashMap :: new () ; map . insert (vec ! [1 , 2] , 45) ; let res = json (& map) . into_response () ; assert_eq ! (res . status () , 500) ; } # [test] fn response_builder_error () { let res = :: http :: Response :: builder () . status (1337) . body ("woops") . into_response () ; assert_eq ! (res . status () , 500) ; } # [test] fn boxed_reply () { let r : Box < dyn Reply > = Box :: new (reply ()) ; let resp = r . into_response () ; assert_eq ! (resp . status () , 200) ; } }
};
}
