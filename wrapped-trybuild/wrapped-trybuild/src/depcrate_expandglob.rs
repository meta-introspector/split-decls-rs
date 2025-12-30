// Generated macro for glob (function)
macro_rules! Depcrate_expandglob {
() => {
// Module: crate::expand
// Provides: {"glob"}
// Dependencies: {}
fn glob (pattern : & str) -> Result < Vec < PathBuf > > { let mut paths = glob :: glob (pattern) ? . map (| entry | entry . map_err (Error :: from)) . collect :: < Result < Vec < PathBuf > > > () ? ; paths . sort () ; Ok (paths) }
};
}
