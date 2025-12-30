// Generated macro for BuilderIncoming (trait)
macro_rules! Depcrate_distBuilderIncoming {
() => {
// Module: crate::dist
// Provides: {"BuilderIncoming"}
// Dependencies: {}
# [cfg (feature = "dist-server")] pub trait BuilderIncoming : Send + Sync { fn run_build (& self , toolchain : Toolchain , command : CompileCommand , outputs : Vec < String > , inputs_rdr : InputsReader < '_ > , cache : & Mutex < TcCache > ,) -> ExtResult < BuildResult , Error > ; }
};
}
