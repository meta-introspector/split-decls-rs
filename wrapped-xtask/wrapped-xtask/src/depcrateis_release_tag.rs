// Generated macro for is_release_tag (function)
macro_rules! Depcrateis_release_tag {
() => {
// Module: crate
// Provides: {"is_release_tag"}
// Dependencies: {}
fn is_release_tag (tag : & str) -> bool { tag . len () == "2020-02-24" . len () && tag . starts_with (| c : char | c . is_ascii_digit ()) }
};
}
