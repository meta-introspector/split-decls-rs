// Generated macro for create_venv_at_path (function)
macro_rules! Depcrate_extra_checkscreate_venv_at_path {
() => {
// Module: crate::extra_checks
// Provides: {"create_venv_at_path"}
// Dependencies: {}
# [doc = " Attempt to create a virtualenv at this path. Cycles through all expected"] # [doc = " valid python versions to find one that is installed."] fn create_venv_at_path (path : & Path) -> Result < () , Error > { # [doc = " Preferred python versions in order. Newest to oldest then current"] # [doc = " development versions"] const TRY_PY : & [& str] = & ["python3.13" , "python3.12" , "python3.11" , "python3.10" , "python3.9" , "python3" , "python" , "python3.14" ,] ; let mut sys_py = None ; let mut found = Vec :: new () ; for py in TRY_PY { match verify_py_version (Path :: new (py)) { Ok (_) => { sys_py = Some (* py) ; break ; } Err (Error :: Io (e)) if e . kind () == io :: ErrorKind :: NotFound => () , Err (Error :: Version { installed , .. }) => found . push (installed) , Err (e) => eprintln ! ("note: error running '{py}': {e}") , } } let Some (sys_py) = sys_py else { let ret = if found . is_empty () { Error :: MissingReq ("python3" , "python file checks" , None) } else { found . sort () ; found . dedup () ; Error :: Version { program : "python3" , required : MIN_PY_REV_STR , installed : found . join (", ") , } } ; return Err (ret) ; } ; if try_create_venv (sys_py , path , "venv") . is_ok () { return Ok (()) ; } try_create_venv (sys_py , path , "virtualenv") }
};
}
