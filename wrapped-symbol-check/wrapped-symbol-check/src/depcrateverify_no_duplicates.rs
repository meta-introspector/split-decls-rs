// Generated macro for verify_no_duplicates (function)
macro_rules! Depcrateverify_no_duplicates {
() => {
// Module: crate
// Provides: {"verify_no_duplicates"}
// Dependencies: {}
# [doc = " Ensure that the same global symbol isn't defined in multiple object files within an archive."] # [doc = ""] # [doc = " Note that this will also locate cases where a symbol is weakly defined in more than one place."] # [doc = " Technically there are no linker errors that will come from this, but it keeps our binary more"] # [doc = " straightforward and saves some distribution size."] fn verify_no_duplicates (archive : & BinFile) { let mut syms = BTreeMap :: < String , SymInfo > :: new () ; let mut dups = Vec :: new () ; let mut found_any = false ; archive . for_each_symbol (| symbol , obj , member | { if ! symbol . is_global () || symbol . is_undefined () { return ; } let sym = SymInfo :: new (& symbol , obj , member) ; if sym . name . starts_with ("__x86.get_pc_thunk") { return ; } if sym . section == ".debug_gdb_scripts" && sym . is_weak { return ; } let win_allowed_dup_pfx = ["__real@" , "__xmm@" , "__ymm@" , "??_C@_" , ".refptr"] ; if win_allowed_dup_pfx . iter () . any (| pfx | sym . name . starts_with (pfx)) { return ; } match syms . get (& sym . name) { Some (existing) => { dups . push (sym) ; dups . push (existing . clone ()) ; } None => { syms . insert (sym . name . clone () , sym) ; } } found_any = true ; }) ; assert ! (found_any , "no symbols found") ; if ! dups . is_empty () { dups . sort_unstable_by (| a , b | a . name . cmp (& b . name)) ; panic ! ("found duplicate symbols: {dups:#?}") ; } println ! ("    success: no duplicate symbols found") ; }
};
}
