// Generated macro for verify_core_symbols (function)
macro_rules! Depcrateverify_core_symbols {
() => {
// Module: crate
// Provides: {"verify_core_symbols"}
// Dependencies: {}
# [doc = " Ensure that there are no references to symbols from `core` that aren't also (somehow) defined."] fn verify_core_symbols (archive : & BinFile) { let mut defined = BTreeSet :: new () ; let mut undefined = Vec :: new () ; let mut has_symbols = false ; archive . for_each_symbol (| symbol , obj , member | { has_symbols = true ; if ! symbol . name () . unwrap () . contains ("_ZN4core") { return ; } let sym = SymInfo :: new (& symbol , obj , member) ; if sym . is_undefined { undefined . push (sym) ; } else { defined . insert (sym . name) ; } }) ; assert ! (has_symbols , "no symbols found") ; undefined . retain (| sym | ! defined . contains (& sym . name)) ; if ! undefined . is_empty () { undefined . sort_unstable_by (| a , b | a . name . cmp (& b . name)) ; panic ! ("found undefined symbols from core: {undefined:#?}") ; } println ! ("    success: no undefined references to core found") ; }
};
}
