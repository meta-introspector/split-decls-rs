// Generated macro for impl_74 (impl)
macro_rules! Depcrate_runnerimpl_74 {
() => {
// Module: crate::runner
// Provides: {"impl_74"}
// Dependencies: {}
impl std :: fmt :: Display for Output { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . spawn . fmt (f) ? ; if let Some (stdout) = & self . stdout { stdout . fmt (f) ? ; } if let Some (stderr) = & self . stderr { stderr . fmt (f) ? ; } self . fs . fmt (f) ? ; Ok (()) } }
};
}
