// Generated macro for check (function)
macro_rules! Depcrate_target_policycheck {
() => {
// Module: crate::target_policy
// Provides: {"check"}
// Dependencies: {}
pub fn check (root_path : & Path , bad : & mut bool) { let mut targets_to_find = HashSet :: new () ; let definitions_path = root_path . join (TARGET_DEFINITIONS_PATH) ; for defn in ignore :: WalkBuilder :: new (& definitions_path) . max_depth (Some (1)) . filter_entry (| e | ! filter_not_rust (e . path ())) . build () { let defn = defn . unwrap () ; if defn . path () == definitions_path { continue ; } let path = defn . path () ; let target_name = path . file_stem () . unwrap () . to_string_lossy () . into_owned () ; let _ = targets_to_find . insert (target_name) ; } walk (& root_path . join (ASSEMBLY_LLVM_TEST_PATH) , | _ , _ | false , & mut | _ , contents | { for line in contents . lines () { let Some (_) = line . find (REVISION_LINE_START) else { continue ; } ; let (_ , target_name) = line . split_at (REVISION_LINE_START . len ()) ; targets_to_find . remove (target_name) ; } }) ; for target in targets_to_find { if ! EXCEPTIONS . contains (& target . as_str ()) { tidy_error ! (bad , "{ASSEMBLY_LLVM_TEST_PATH}: missing assembly test for {target}") } } }
};
}
