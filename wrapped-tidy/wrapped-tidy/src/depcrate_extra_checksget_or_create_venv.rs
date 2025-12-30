// Generated macro for get_or_create_venv (function)
macro_rules! Depcrate_extra_checksget_or_create_venv {
() => {
// Module: crate::extra_checks
// Provides: {"get_or_create_venv"}
// Dependencies: {}
# [doc = " Create a virtuaenv at a given path if it doesn't already exist, or validate"] # [doc = " the install if it does. Returns the path to that venv's python executable."] fn get_or_create_venv (venv_path : & Path , src_reqs_path : & Path) -> Result < PathBuf , Error > { let mut should_create = true ; let dst_reqs_path = venv_path . join ("requirements.txt") ; let mut py_path = venv_path . to_owned () ; py_path . extend (REL_PY_PATH) ; if let Ok (req) = fs :: read_to_string (& dst_reqs_path) { if req == fs :: read_to_string (src_reqs_path) ? { should_create = false ; } else { eprintln ! ("requirements.txt file mismatch, recreating environment") ; } } if should_create { eprintln ! ("removing old virtual environment") ; if venv_path . is_dir () { fs :: remove_dir_all (venv_path) . unwrap_or_else (| _ | { panic ! ("failed to remove directory at {}" , venv_path . display ()) }) ; } create_venv_at_path (venv_path) ? ; install_requirements (& py_path , src_reqs_path , & dst_reqs_path) ? ; } verify_py_version (& py_path) ? ; Ok (py_path) }
};
}
