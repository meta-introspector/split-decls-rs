// Generated macro for strip_lines (function)
macro_rules! Depcratestrip_lines {
() => {
// Module: crate
// Provides: {"strip_lines"}
// Dependencies: {}
fn strip_lines (data : & str , starts_with : & str) -> String { data . lines () . filter (| line | ! line . trim_start () . starts_with (starts_with)) . collect :: < Vec < _ > > () . join ("\n") }
};
}
