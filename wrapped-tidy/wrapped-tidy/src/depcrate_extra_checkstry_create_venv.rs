// Generated macro for try_create_venv (function)
macro_rules! Depcrate_extra_checkstry_create_venv {
() => {
// Module: crate::extra_checks
// Provides: {"try_create_venv"}
// Dependencies: {}
fn try_create_venv (python : & str , path : & Path , module : & str) -> Result < () , Error > { eprintln ! ("creating virtual environment at '{}' using '{python}' and '{module}'" , path . display ()) ; let out = Command :: new (python) . args (["-m" , module]) . arg (path) . output () . unwrap () ; if out . status . success () { return Ok (()) ; } let stderr = String :: from_utf8_lossy (& out . stderr) ; let err = if stderr . contains (& format ! ("No module named {module}")) { Error :: Generic (format ! (r#"{module} not found: you may need to install it:
`{python} -m pip install {module}`
If you see an error about "externally managed environment" when running the above command,
either install `{module}` using your system package manager
(e.g. `sudo apt-get install {python}-{module}`) or create a virtual environment manually, install
`{module}` in it and then activate it before running tidy.
"#)) } else { Error :: Generic (format ! ("failed to create venv at '{}' using {python} -m {module}: {stderr}" , path . display ())) } ; Err (err) }
};
}
