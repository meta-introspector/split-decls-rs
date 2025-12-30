// Generated macro for python (function)
macro_rules! Depcratepython {
() => {
// Module: crate
// Provides: {"python"}
// Dependencies: {}
fn python () -> & 'static str { let Some (path) = env :: var_os ("PATH") else { return PYTHON ; } ; let mut python2 = false ; let mut python3 = false ; for dir in env :: split_paths (& path) { if dir . join (PYTHON) . with_extension (EXE_EXTENSION) . exists () { return PYTHON ; } python2 |= dir . join (PYTHON2) . with_extension (EXE_EXTENSION) . exists () ; python3 |= dir . join (PYTHON3) . with_extension (EXE_EXTENSION) . exists () ; } if python3 { PYTHON3 } else if python2 { PYTHON2 } else { eprintln ! ("Unable to find python in your PATH. Please check it is installed.") ; process :: exit (1) ; } }
};
}
