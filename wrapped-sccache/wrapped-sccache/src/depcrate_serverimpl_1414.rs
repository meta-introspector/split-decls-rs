// Generated macro for impl_1414 (impl)
macro_rules! Depcrate_serverimpl_1414 {
() => {
// Module: crate::server
// Provides: {"impl_1414"}
// Dependencies: {}
impl PerLanguageCount { fn increment (& mut self , kind : & CompilerKind , lang : & Language) { let lang_comp_key = kind . lang_comp_kind (lang) ; let adv_count = self . adv_counts . entry (lang_comp_key) . or_insert (0) ; * adv_count += 1 ; let lang_key = kind . lang_kind (lang) ; let count = self . counts . entry (lang_key) . or_insert (0) ; * count += 1 ; } pub fn all (& self) -> u64 { self . counts . values () . sum () } pub fn get (& self , key : & str) -> Option < & u64 > { self . counts . get (key) } pub fn get_adv (& self , key : & str) -> Option < & u64 > { self . adv_counts . get (key) } pub fn new () -> PerLanguageCount { Self :: default () } }
};
}
