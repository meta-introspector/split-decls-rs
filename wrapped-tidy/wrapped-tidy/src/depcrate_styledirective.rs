// Generated macro for Directive (enum)
macro_rules! Depcrate_styleDirective {
() => {
// Module: crate::style
// Provides: {"Directive"}
// Dependencies: {}
# [derive (Clone , Copy)] enum Directive { # [doc = " By default, tidy always warns against style issues."] Deny , # [doc = " `Ignore(false)` means that an `ignore-tidy-*` directive"] # [doc = " has been provided, but is unnecessary. `Ignore(true)`"] # [doc = " means that it is necessary (i.e. a warning would be"] # [doc = " produced if `ignore-tidy-*` was not present)."] Ignore (bool) , }
};
}
