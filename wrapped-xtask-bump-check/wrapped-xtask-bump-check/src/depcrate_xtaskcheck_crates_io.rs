// Generated macro for check_crates_io (function)
macro_rules! Depcrate_xtaskcheck_crates_io {
() => {
// Module: crate::xtask
// Provides: {"check_crates_io"}
// Dependencies: {}
# [doc = " Compares version against published crates on crates.io."] # [doc = ""] # [doc = " Assumption: We always release a version larger than all existing versions."] fn check_crates_io < 'a > (ws : & Workspace < 'a > , changed_members : & HashMap < & 'a str , & 'a Package > , needs_bump : & mut Vec < & 'a Package > ,) -> CargoResult < () > { let gctx = ws . gctx () ; let source_id = SourceId :: crates_io (gctx) ? ; let mut registry = ws . package_registry () ? ; let _lock = gctx . acquire_package_cache_lock (CacheLockMode :: DownloadExclusive) ? ; registry . lock_patches () ; gctx . shell () . status (STATUS , format_args ! ("compare against `{}`" , source_id . display_registry_name ()) ,) ? ; for (name , member) in changed_members { let current = member . version () ; let version_req = format ! (">={current}") ; let query = Dependency :: parse (* name , Some (& version_req) , source_id) ? ; let possibilities = loop { match registry . query_vec (& query , QueryKind :: Exact) { task :: Poll :: Ready (res) => { break res ? ; } task :: Poll :: Pending => registry . block_until_ready () ? , } } ; if possibilities . is_empty () { tracing :: trace ! ("dep `{name}` has no version greater than or equal to `{current}`") ; } else { tracing :: trace ! ("`{name}@{current}` needs a bump because its should have a version newer than crates.io: {:?}`" , possibilities . iter () . map (| s | s . as_summary ()) . map (| s | format ! ("{}@{}" , s . name () , s . version ())) . collect ::< Vec < _ >> () ,) ; needs_bump . push (member) ; } } Ok (()) }
};
}
