// Generated macro for set_to_summary_str (function)
macro_rules! Depcrateset_to_summary_str {
() => {
// Module: crate
// Provides: {"set_to_summary_str"}
// Dependencies: {}
fn set_to_summary_str (set : & BTreeSet < String > , dir : & str) -> String { set . iter () . map (| ref n | format ! ("    - [{}]({}/{}.md)" , n . replace ('-' , "_") , dir , n)) . fold ("" . to_owned () , | s , a | s + & a + "\n") }
};
}
