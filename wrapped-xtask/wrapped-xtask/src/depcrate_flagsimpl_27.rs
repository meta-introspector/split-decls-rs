// Generated macro for impl_27 (impl)
macro_rules! Depcrate_flagsimpl_27 {
() => {
// Module: crate::flags
// Provides: {"impl_27"}
// Dependencies: {}
impl Install { pub (crate) fn server (& self) -> Option < ServerOpt > { if (self . client || self . proc_macro_server) && ! self . server { return None ; } let malloc = if self . mimalloc { Malloc :: Mimalloc } else if self . jemalloc { Malloc :: Jemalloc } else if self . enable_profiling { Malloc :: Dhat } else { Malloc :: System } ; Some (ServerOpt { malloc , dev_rel : self . dev_rel || self . enable_profiling , pgo : self . pgo . clone () , force_always_assert : self . force_always_assert , }) } pub (crate) fn proc_macro_server (& self) -> Option < ProcMacroServerOpt > { if ! self . proc_macro_server { return None ; } Some (ProcMacroServerOpt { dev_rel : self . dev_rel }) } pub (crate) fn client (& self) -> Option < ClientOpt > { if (self . server || self . proc_macro_server) && ! self . client { return None ; } Some (ClientOpt { code_bin : self . code_bin . clone () }) } }
};
}
