// Generated macro for LinkSelfContainedDefault (enum)
macro_rules! Depcrate_specLinkSelfContainedDefault {
() => {
// Module: crate::spec
// Provides: {"LinkSelfContainedDefault"}
// Dependencies: {}
# [doc = " The different `-Clink-self-contained` options that can be specified in a target spec:"] # [doc = " - enabling or disabling in bulk"] # [doc = " - some target-specific pieces of inference to determine whether to use self-contained linking"] # [doc = "   if `-Clink-self-contained` is not specified explicitly (e.g. on musl/mingw)"] # [doc = " - explicitly enabling some of the self-contained linking components, e.g. the linker component"] # [doc = "   to use `rust-lld`"] # [derive (Clone , Copy , PartialEq , Debug)] pub enum LinkSelfContainedDefault { # [doc = " The target spec explicitly enables self-contained linking."] True , # [doc = " The target spec explicitly disables self-contained linking."] False , # [doc = " The target spec requests that the self-contained mode is inferred, in the context of musl."] InferredForMusl , # [doc = " The target spec requests that the self-contained mode is inferred, in the context of mingw."] InferredForMingw , # [doc = " The target spec explicitly enables a list of self-contained linking components: e.g. for"] # [doc = " targets opting into a subset of components like the CLI's `-C link-self-contained=+linker`."] WithComponents (LinkSelfContainedComponents) , }
};
}
