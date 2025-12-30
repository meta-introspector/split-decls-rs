// Generated macro for verify_py_version (function)
macro_rules! Depcrate_extra_checksverify_py_version {
() => {
// Module: crate::extra_checks
// Provides: {"verify_py_version"}
// Dependencies: {}
# [doc = " Parse python's version output (`Python x.y.z`) and ensure we have a"] # [doc = " suitable version."] fn verify_py_version (py_path : & Path) -> Result < () , Error > { let out = Command :: new (py_path) . arg ("--version") . output () ? ; let outstr = String :: from_utf8_lossy (& out . stdout) ; let vers = outstr . trim () . split_ascii_whitespace () . nth (1) . unwrap () . trim () ; let mut vers_comps = vers . split ('.') ; let major : u32 = vers_comps . next () . unwrap () . parse () . unwrap () ; let minor : u32 = vers_comps . next () . unwrap () . parse () . unwrap () ; if (major , minor) < MIN_PY_REV { Err (Error :: Version { program : "python" , required : MIN_PY_REV_STR , installed : vers . to_owned () , }) } else { Ok (()) } }
};
}
