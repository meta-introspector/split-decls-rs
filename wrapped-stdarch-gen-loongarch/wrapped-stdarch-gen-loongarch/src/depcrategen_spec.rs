// Generated macro for gen_spec (function)
macro_rules! Depcrategen_spec {
() => {
// Module: crate
// Provides: {"gen_spec"}
// Dependencies: {}
fn gen_spec (in_file : String , ext_name : & str) -> io :: Result < () > { let f = File :: open (in_file . clone ()) . unwrap_or_else (| _ | panic ! ("Failed to open {in_file}")) ; let f = BufReader :: new (f) ; let mut out = format ! (r#"// This code is automatically generated. DO NOT MODIFY.
// ```
// OUT_DIR=`pwd`/crates/stdarch-gen-loongarch cargo run -p stdarch-gen-loongarch -- {in_file}
// ```
"#) ; out . push ('\n') ; let mut asm_fmts = String :: new () ; let mut data_types = String :: new () ; let fn_pat = format ! ("__{ext_name}_") ; for line in f . lines () { let line = line . unwrap () ; if line . is_empty () { continue ; } if let Some (s) = line . find ("/* Assembly instruction format:") { let e = line . find ('.') . unwrap () ; asm_fmts = line . get (s + 31 .. e) . unwrap () . trim () . to_string () ; } else if let Some (s) = line . find ("/* Data types in instruction templates:") { let e = line . find ('.') . unwrap () ; data_types = line . get (s + 39 .. e) . unwrap () . trim () . to_string () ; } else if let Some (s) = line . find (fn_pat . as_str ()) { let e = line . find ('(') . unwrap () ; let name = line . get (s + 2 .. e) . unwrap () . trim () . to_string () ; out . push_str (& format ! ("/// {name}\n")) ; out . push_str (& format ! ("name = {name}\n")) ; out . push_str (& format ! ("asm-fmts = {asm_fmts}\n")) ; out . push_str (& format ! ("data-types = {data_types}\n")) ; out . push ('\n') ; } } let out_dir_path : PathBuf = PathBuf :: from (env :: var ("OUT_DIR") . unwrap ()) ; std :: fs :: create_dir_all (& out_dir_path) ? ; let mut f = File :: create (out_dir_path . join (format ! ("{ext_name}.spec"))) ? ; f . write_all (out . as_bytes ()) ? ; Ok (()) }
};
}
