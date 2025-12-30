// Generated macro for impl_75 (impl)
macro_rules! Depcrate_installimpl_75 {
() => {
// Module: crate::install
// Provides: {"impl_75"}
// Dependencies: {}
impl flags :: Install { pub (crate) fn run (self , sh : & Shell) -> anyhow :: Result < () > { if cfg ! (target_os = "macos") { fix_path_for_mac (sh) . context ("Fix path for mac") ? ; } if let Some (server) = self . server () { install_server (sh , server) . context ("install server") ? ; } if let Some (server) = self . proc_macro_server () { install_proc_macro_server (sh , server) . context ("install proc-macro server") ? ; } if let Some (client) = self . client () { install_client (sh , client) . context ("install client") ? ; } Ok (()) } }
};
}
