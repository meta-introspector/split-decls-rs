// Generated macro for impl_109 (impl)
macro_rules! Depcrate_rustimpl_109 {
() => {
// Module: crate::rust
// Provides: {"impl_109"}
// Dependencies: {}
impl LanguageMethods for Rust { fn display (& self) -> & str { "rust" } fn comment_prefix_for_test_config (& self) -> Option < & str > { Some ("//@") } fn should_fail_verify (& self , name : & str , config : & crate :: config :: WitConfig , args : & [String] ,) -> bool { if config . async_ && args . iter () . any (| s | s == "--std-feature") { return true ; } if name == "wasi-http-borrowed-duplicate" { return true ; } false } fn codegen_test_variants (& self) -> & [(& str , & [& str])] { & [("borrowed" , & ["--ownership=borrowing"]) , ("borrowed-duplicate" , & ["--ownership=borrowing-duplicate-if-necessary"] ,) , ("async" , & ["--async=all"]) , ("no-std" , & ["--std-feature"]) ,] } fn default_bindgen_args (& self) -> & [& str] { & ["--generate-all" , "--format"] } fn default_bindgen_args_for_codegen (& self) -> & [& str] { & ["--stubs"] } fn prepare (& self , runner : & mut Runner < '_ >) -> Result < () > { let cwd = env :: current_dir () ? ; let opts = & runner . opts . rust ; let dir = cwd . join (& runner . opts . artifacts) . join ("rust") ; let wit_bindgen = dir . join ("wit-bindgen") ; let wit_bindgen_dep = match & opts . rust_wit_bindgen_path { Some (path) => format ! ("path = {:?}" , cwd . join (path)) , None => { let version = opts . rust_wit_bindgen_version . as_deref () . unwrap_or (env ! ("CARGO_PKG_VERSION")) ; format ! ("version = \"{version}\"") } } ; super :: write_if_different (& wit_bindgen . join ("Cargo.toml") , & format ! (r#"
[package]
name = "tmp"

[workspace]

[dependencies]
wit-bindgen = {{ {wit_bindgen_dep}, features = ['async-spawn'] }}
futures = "0.3.31"

[lib]
path = 'lib.rs'
            "# ,) ,) ? ; super :: write_if_different (& wit_bindgen . join ("lib.rs") , "") ? ; println ! ("Building `wit-bindgen` from crates.io...") ; runner . run_command (Command :: new ("cargo") . current_dir (& wit_bindgen) . arg ("build") . arg ("-pwit-bindgen") . arg ("-pfutures") . arg ("--target") . arg (& opts . rust_target) ,) ? ; let target_out_dir = wit_bindgen . join ("target") . join (& opts . rust_target) . join ("debug") ; let host_out_dir = wit_bindgen . join ("target/debug") ; let wit_bindgen_rlib = target_out_dir . join ("libwit_bindgen.rlib") ; let futures_rlib = target_out_dir . join ("libfutures.rlib") ; assert ! (wit_bindgen_rlib . exists ()) ; assert ! (futures_rlib . exists ()) ; runner . rust_state = Some (State { wit_bindgen_rlib , futures_rlib , wit_bindgen_deps : vec ! [target_out_dir . join ("deps") , host_out_dir . join ("deps")] , }) ; Ok (()) } fn compile (& self , runner : & Runner < '_ > , compile : & Compile) -> Result < () > { let config = compile . component . deserialize_lang_config :: < RustConfig > () ? ; let output = if runner . produces_component () { compile . output . to_path_buf () } else { compile . output . with_extension ("core.wasm") } ; let mut externs = Vec :: new () ; let manifest_dir = compile . component . path . parent () . unwrap () ; let rustc = | path : & Path , output : & Path | { let mut cmd = runner . rustc (Edition :: E2021) ; cmd . env ("CARGO_MANIFEST_DIR" , manifest_dir) . arg (path) . arg ("-o") . arg (& output) ; for flag in Vec :: from (config . rustflags . clone ()) { cmd . arg (flag) ; } cmd } ; for file in config . externs . iter () { let file = manifest_dir . join (file) ; let stem = file . file_stem () . unwrap () . to_str () . unwrap () ; let output = compile . artifacts_dir . join (format ! ("lib{stem}.rlib")) ; runner . run_command (rustc (& file , & output) . arg ("--crate-type=rlib")) ? ; externs . push ((stem . to_string () , output)) ; } let mut cmd = rustc (& compile . component . path , & output) ; cmd . env ("BINDINGS" , compile . bindings_dir . join (format ! ("{}.rs" , compile . component . bindgen . world . replace ('-' , "_"))) ,) ; for (name , path) in externs { let arg = format ! ("--extern={name}={}" , path . display ()) ; cmd . arg (arg) ; } match compile . component . kind { Kind :: Runner => { } Kind :: Test => { cmd . arg ("--crate-type=cdylib") ; } } runner . run_command (& mut cmd) ? ; if ! runner . produces_component () { runner . convert_p1_to_component (& output , compile) . with_context (| | format ! ("failed to convert {output:?}")) ? ; } Ok (()) } fn verify (& self , runner : & Runner < '_ > , verify : & Verify < '_ >) -> Result < () > { let bindings = verify . bindings_dir . join (format ! ("{}.rs" , verify . world . to_snake_case ())) ; let test_edition = | edition : Edition | -> Result < () > { let mut cmd = runner . rustc (edition) ; cmd . arg (& bindings) . arg ("--crate-type=rlib") . arg ("-o") . arg (verify . artifacts_dir . join ("tmp")) ; runner . run_command (& mut cmd) ? ; Ok (()) } ; test_edition (Edition :: E2021) ? ; test_edition (Edition :: E2024) ? ; if verify . args . iter () . any (| s | s == "--std-feature") { let no_std_root = verify . artifacts_dir . join ("no_std.rs") ; super :: write_if_different (& no_std_root , r#"
#![no_std]
include!(env!("BINDINGS"));

// This empty module named 'core' is here to catch module path
// conflicts with 'core' modules used in code generated by the
// wit_bindgen::generate macro.
// Ref: https://github.com/bytecodealliance/wit-bindgen/pull/568
mod core {}
                "# ,) ? ; let mut cmd = runner . rustc (Edition :: E2021) ; cmd . arg (& no_std_root) . env ("BINDINGS" , & bindings) . arg ("--crate-type=rlib") . arg ("-o") . arg (verify . artifacts_dir . join ("tmp")) ; runner . run_command (& mut cmd) ? ; } Ok (()) } }
};
}
