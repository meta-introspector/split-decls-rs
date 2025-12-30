// Generated macro for FixtureWithProjectMeta (struct)
macro_rules! Depcrate_fixtureFixtureWithProjectMeta {
() => {
// Module: crate::fixture
// Provides: {"FixtureWithProjectMeta"}
// Dependencies: {}
# [derive (Debug)] pub struct FixtureWithProjectMeta { pub fixture : Vec < Fixture > , pub mini_core : Option < MiniCore > , pub proc_macro_names : Vec < String > , pub toolchain : Option < String > , # [doc = " Specifies LLVM data layout to be used."] # [doc = ""] # [doc = " You probably don't want to manually specify this. See LLVM manual for the"] # [doc = " syntax, if you must: <https://llvm.org/docs/LangRef.html#data-layout>"] pub target_data_layout : String , # [doc = " Specifies the target architecture."] pub target_arch : String , }
};
}
