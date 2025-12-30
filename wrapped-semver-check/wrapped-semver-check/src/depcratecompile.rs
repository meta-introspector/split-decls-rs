// Generated macro for compile (function)
macro_rules! Depcratecompile {
() => {
// Module: crate
// Provides: {"compile"}
// Dependencies: {}
fn compile (contents : & str , path : & Path , crate_name : & str , extern_path : bool ,) -> Result < Output , Box < dyn Error > > { let crate_type = if contents . contains ("fn main()") { "bin" } else { "rlib" } ; fs :: write (path , & contents) ? ; check_formatting (path) ? ; let out_dir = path . parent () . unwrap () ; let mut cmd = Command :: new ("rustc") ; cmd . args (& ["--edition=2021" , "--crate-type" , crate_type , "--crate-name" , crate_name , "--out-dir" ,]) ; cmd . arg (& out_dir) ; if extern_path { let epath = out_dir . join (format ! ("lib{}.rlib" , CRATE_NAME)) ; cmd . arg ("--extern") . arg (format ! ("{}={}" , CRATE_NAME , epath . display ())) ; } cmd . arg (path) ; cmd . output () . map_err (Into :: into) }
};
}
