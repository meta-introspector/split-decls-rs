// Generated macro for tests (module)
macro_rules! Depcrate_zone_windowstests {
() => {
// Module: crate::zone::windows
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use icu :: locale :: subtags :: subtag ; # [test] fn basic_windows_tz_lookup () { let win_map = WindowsParser :: new () ; let result = win_map . parse ("Central Standard Time" , None) ; assert_eq ! (result , Some (TimeZone (subtag ! ("uschi")))) ; let result = win_map . parse ("Eastern Standard Time" , None) ; assert_eq ! (result , Some (TimeZone (subtag ! ("usnyc")))) ; let result = win_map . parse ("Eastern Standard Time" , Some (region ! ("CA"))) ; assert_eq ! (result , Some (TimeZone (subtag ! ("cator")))) ; let result = win_map . parse ("GMT Standard Time" , None) ; assert_eq ! (result , Some (TimeZone (subtag ! ("gblon")))) ; } }
};
}
