// Generated macro for description (function)
macro_rules! Depcrate_wycheproofdescription {
() => {
// Module: crate::wycheproof
// Provides: {"description"}
// Dependencies: {}
# [doc = " Build a description for a test case in a suite"] pub fn description (suite : & Suite , case : & Case) -> String { format ! ("{} case {} [{}] {}" , suite . algorithm , case . case_id , case . result , case . comment) }
};
}
