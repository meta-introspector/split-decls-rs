// Generated macro for find_sccache_binary (function)
macro_rules! Depcrate_test_utilsfind_sccache_binary {
() => {
// Module: crate::test::utils
// Provides: {"find_sccache_binary"}
// Dependencies: {}
# [cfg (not (target_os = "macos"))] pub fn find_sccache_binary () -> PathBuf { let exe = env :: current_exe () . unwrap () ; let this_dir = exe . parent () . unwrap () ; let dirs = & [& this_dir , & this_dir . parent () . unwrap ()] ; dirs . iter () . map (| d | d . join ("sccache") . with_extension (env :: consts :: EXE_EXTENSION)) . filter_map (| d | fs :: metadata (& d) . ok () . map (| _ | d)) . next () . unwrap_or_else (| | { panic ! ("Error: sccache binary not found, looked in `{:?}`. Do you need to run `cargo build`?" , dirs) }) }
};
}
