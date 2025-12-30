// Generated macro for gen_bind (function)
macro_rules! Depcrategen_bind {
() => {
// Module: crate
// Provides: {"gen_bind"}
// Dependencies: {}
fn gen_bind (in_file : String , ext_name : & str) -> io :: Result < () > { let f = File :: open (in_file . clone ()) . unwrap_or_else (| _ | panic ! ("Failed to open {in_file}")) ; let f = BufReader :: new (f) ; let target : TargetFeature = TargetFeature :: new (ext_name) ; let mut para_num ; let mut current_name : Option < String > = None ; let mut asm_fmts : Vec < String > = Vec :: new () ; let mut link_function_str = String :: new () ; let mut function_str = String :: new () ; let mut out = String :: new () ; out . push_str (& format ! (r#"// This code is automatically generated. DO NOT MODIFY.
//
// Instead, modify `{in_file}` and run the following command to re-generate this file:
//
// ```
// OUT_DIR=`pwd`/crates/core_arch cargo run -p stdarch-gen-loongarch -- {in_file}
// ```

use crate::mem::transmute;
use super::types::*;
"#)) ; out . push_str (r#"
#[allow(improper_ctypes)]
unsafe extern "unadjusted" {
"# ,) ; for line in f . lines () { let line = line . unwrap () ; if line . is_empty () { continue ; } if let Some (name) = line . strip_prefix ("name = ") { current_name = Some (String :: from (name)) ; } else if line . starts_with ("asm-fmts = ") { asm_fmts = line [10 ..] . split (',') . map (| v | v . trim () . to_string ()) . collect () ; } else if line . starts_with ("data-types = ") { let current_name = current_name . clone () . unwrap () ; let data_types : Vec < & str > = line . get (12 ..) . unwrap () . split (',') . map (| e | e . trim ()) . collect () ; let in_t ; let out_t ; if data_types . len () == 2 { in_t = [data_types [1] , "NULL" , "NULL" , "NULL"] ; out_t = data_types [0] ; para_num = 1 ; } else if data_types . len () == 3 { in_t = [data_types [1] , data_types [2] , "NULL" , "NULL"] ; out_t = data_types [0] ; para_num = 2 ; } else if data_types . len () == 4 { in_t = [data_types [1] , data_types [2] , data_types [3] , "NULL"] ; out_t = data_types [0] ; para_num = 3 ; } else if data_types . len () == 5 { in_t = [data_types [1] , data_types [2] , data_types [3] , data_types [4]] ; out_t = data_types [0] ; para_num = 4 ; } else { panic ! ("DEBUG: line: {0} len: {1}" , line , data_types . len ()) ; } let (link_function , function) = gen_bind_body (& current_name , & asm_fmts , & in_t , out_t , para_num , target) ; link_function_str . push_str (& link_function) ; function_str . push_str (& function) ; } } out . push_str (& link_function_str) ; out . push_str ("}\n") ; out . push_str (& function_str) ; let out_path : PathBuf = PathBuf :: from (env :: var ("OUT_DIR") . unwrap_or ("crates/core_arch" . to_string ())) . join ("src") . join ("loongarch64") . join (ext_name) ; std :: fs :: create_dir_all (& out_path) ? ; let mut file = File :: create (out_path . join ("generated.rs")) ? ; file . write_all (out . as_bytes ()) ? ; Ok (()) }
};
}
