// Generated macro for run_ruff (function)
macro_rules! Depcrate_extra_checksrun_ruff {
() => {
// Module: crate::extra_checks
// Provides: {"run_ruff"}
// Dependencies: {}
fn run_ruff (root_path : & Path , outdir : & Path , py_path : & Path , cfg_args : & [& OsStr] , file_args : & [& OsStr] , ruff_args : & [& OsStr] ,) -> Result < () , Error > { let mut cfg_args_ruff = cfg_args . to_vec () ; let mut file_args_ruff = file_args . to_vec () ; let mut cfg_path = root_path . to_owned () ; cfg_path . extend (RUFF_CONFIG_PATH) ; let mut cache_dir = outdir . to_owned () ; cache_dir . extend (RUFF_CACHE_PATH) ; cfg_args_ruff . extend (["--config" . as_ref () , cfg_path . as_os_str () , "--cache-dir" . as_ref () , cache_dir . as_os_str () ,]) ; if file_args_ruff . is_empty () { file_args_ruff . push (root_path . as_os_str ()) ; } let mut args : Vec < & OsStr > = ruff_args . to_vec () ; args . extend (merge_args (& cfg_args_ruff , & file_args_ruff)) ; py_runner (py_path , true , None , "ruff" , & args) }
};
}
