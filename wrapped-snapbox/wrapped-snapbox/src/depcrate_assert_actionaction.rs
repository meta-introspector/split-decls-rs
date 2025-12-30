// Generated macro for Action (enum)
macro_rules! Depcrate_assert_actionAction {
() => {
// Module: crate::assert::action
// Provides: {"Action"}
// Dependencies: {}
# [doc = " Test action, see [`Assert`][crate::Assert]"] # [derive (Copy , Clone , Debug , PartialEq , Eq , Default)] pub enum Action { # [doc = " Do not run the test"] Skip , # [doc = " Ignore test failures"] Ignore , # [doc = " Fail on mismatch"] # [default] Verify , # [doc = " Overwrite on mismatch"] Overwrite , }
};
}
