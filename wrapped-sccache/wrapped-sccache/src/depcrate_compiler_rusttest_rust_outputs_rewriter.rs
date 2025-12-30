// Generated macro for test_rust_outputs_rewriter (function)
macro_rules! Depcrate_compiler_rusttest_rust_outputs_rewriter {
() => {
// Module: crate::compiler::rust
// Provides: {"test_rust_outputs_rewriter"}
// Dependencies: {}
# [test] # [cfg (all (feature = "dist-client" , target_os = "windows"))] fn test_rust_outputs_rewriter () { use crate :: compiler :: compiler :: OutputsRewriter ; use crate :: test :: utils :: create_file ; use std :: io :: Write ; let mut pt = dist :: PathTransformer :: new () ; pt . as_dist (Path :: new ("c:\\")) . unwrap () ; let mappings : Vec < _ > = pt . disk_mappings () . collect () ; assert ! (mappings . len () == 1) ; let linux_prefix = & mappings [0] . 1 ; let depinfo_data = format ! ("{prefix}/sccache/target/x86_64-unknown-linux-gnu/debug/deps/sccache_dist-c6f3229b9ef0a5c3.rmeta: src/bin/sccache-dist/main.rs src/bin/sccache-dist/build.rs src/bin/sccache-dist/token_check.rs

{prefix}/sccache/target/x86_64-unknown-linux-gnu/debug/deps/sccache_dist-c6f3229b9ef0a5c3.d: src/bin/sccache-dist/main.rs src/bin/sccache-dist/build.rs src/bin/sccache-dist/token_check.rs

src/bin/sccache-dist/main.rs:
src/bin/sccache-dist/build.rs:
src/bin/sccache-dist/token_check.rs:
" , prefix = linux_prefix) ; let depinfo_resulting_data = format ! ("{prefix}/sccache/target/x86_64-unknown-linux-gnu/debug/deps/sccache_dist-c6f3229b9ef0a5c3.rmeta: src/bin/sccache-dist/main.rs src/bin/sccache-dist/build.rs src/bin/sccache-dist/token_check.rs

{prefix}/sccache/target/x86_64-unknown-linux-gnu/debug/deps/sccache_dist-c6f3229b9ef0a5c3.d: src/bin/sccache-dist/main.rs src/bin/sccache-dist/build.rs src/bin/sccache-dist/token_check.rs

src/bin/sccache-dist/main.rs:
src/bin/sccache-dist/build.rs:
src/bin/sccache-dist/token_check.rs:
" , prefix = "c:") ; let tempdir = tempfile :: Builder :: new () . prefix ("sccache_test") . tempdir () . unwrap () ; let tempdir = tempdir . path () ; let depinfo_file = create_file (tempdir , "depinfo.d" , | mut f | { f . write_all (depinfo_data . as_bytes ()) }) . unwrap () ; let ror = Box :: new (RustOutputsRewriter { dep_info : Some (depinfo_file . clone ()) , }) ; let () = ror . handle_outputs (& pt , & [depinfo_file . clone ()] , & []) . unwrap () ; let mut s = String :: new () ; fs :: File :: open (depinfo_file) . unwrap () . read_to_string (& mut s) . unwrap () ; assert_eq ! (s , depinfo_resulting_data) }
};
}
