// Generated macro for is_excluded (function)
macro_rules! Depcrateis_excluded {
() => {
// Module: crate
// Provides: {"is_excluded"}
// Dependencies: {}
fn is_excluded (archive : & Path) -> bool { let mut lut = EXCLUDE_LUT . lock () ; lut . as_mut () . and_then (| cache | { let archive = env :: current_dir () . ok () ? . join (archive) ; let relative_path = archive . strip_prefix (cache . base ()) . ok () ? ; cache . at_path (relative_path , Some (gix_worktree :: index :: entry :: Mode :: FILE) , & gix_worktree :: object :: find :: Never ,) . ok () ? . is_excluded () . into () }) . unwrap_or (false) }
};
}
