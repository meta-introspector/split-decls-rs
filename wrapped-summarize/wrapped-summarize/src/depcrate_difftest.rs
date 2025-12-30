// Generated macro for test (module)
macro_rules! Depcrate_difftest {
() => {
// Module: crate::diff
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: SignedDuration ; use std :: time :: Duration ; # [test] fn op_subtract () { let zero_d = Duration :: from_nanos (0) ; let one_d = Duration :: from_nanos (1) ; let two_d = Duration :: from_nanos (2) ; let zero_sd = SignedDuration :: from (zero_d) ; let one_sd = SignedDuration :: from (one_d) ; let neg_one_sd = SignedDuration { duration : one_d , is_positive : false , } ; let two_sd = SignedDuration :: from (two_d) ; let neg_two_sd = SignedDuration { duration : two_d , is_positive : false , } ; assert_eq ! (zero_d , zero_sd . duration) ; assert_eq ! (true , zero_sd . is_positive) ; assert_eq ! (zero_sd , zero_sd - zero_sd) ; assert_eq ! (one_d , one_sd . duration) ; assert_eq ! (true , one_sd . is_positive) ; assert_eq ! (one_sd , one_sd - zero_sd) ; assert_eq ! (one_d , neg_one_sd . duration) ; assert_eq ! (false , neg_one_sd . is_positive) ; assert_eq ! (neg_one_sd , neg_one_sd - zero_sd) ; assert_eq ! (zero_sd , one_sd - one_sd) ; assert_eq ! (one_sd , two_sd - one_sd) ; assert_eq ! (neg_one_sd , one_sd - two_sd) ; assert_eq ! (neg_two_sd , neg_one_sd - one_sd) ; assert_eq ! (zero_sd , neg_one_sd - neg_one_sd) ; } }
};
}
