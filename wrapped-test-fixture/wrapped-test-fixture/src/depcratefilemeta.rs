// Generated macro for FileMeta (struct)
macro_rules! DepcrateFileMeta {
() => {
// Module: crate
// Provides: {"FileMeta"}
// Dependencies: {}
# [derive (Debug)] struct FileMeta { path : String , krate : Option < (String , CrateOrigin , Option < String >) > , deps : Vec < String > , extern_prelude : Option < Vec < String > > , cfg : CfgOptions , edition : Edition , env : Env , introduce_new_source_root : Option < SourceRootKind > , }
};
}
