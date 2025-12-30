// Generated macro for trim_start_matches (function)
macro_rules! Depcratetrim_start_matches {
() => {
// Module: crate
// Provides: {"trim_start_matches"}
// Dependencies: {}
# [doc = " `trim_left_matches` has been deprecated in favor of `trim_start_matches`."] # [doc = " This helper silences the warning, as we need to continue using"] # [doc = " `trim_left_matches` for rust 1.15 support."] # [allow (deprecated)] fn trim_start_matches (s : & str , c : char) -> & str { s . trim_left_matches (c) }
};
}
