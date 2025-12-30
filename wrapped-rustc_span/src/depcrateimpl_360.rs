// Generated macro for impl_360 (impl)
macro_rules! Depcrateimpl_360 {
() => {
// Module: crate
// Provides: {"impl_360"}
// Dependencies: {}
impl fmt :: Display for FileNameDisplay < '_ > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use FileName :: * ; match * self . inner { Real (ref name) => { write ! (fmt , "{}" , name . to_string_lossy (self . display_pref)) } CfgSpec (_) => write ! (fmt , "<cfgspec>") , MacroExpansion (_) => write ! (fmt , "<macro expansion>") , Anon (_) => write ! (fmt , "<anon>") , ProcMacroSourceCode (_) => write ! (fmt , "<proc-macro source code>") , CliCrateAttr (_) => write ! (fmt , "<crate attribute>") , Custom (ref s) => write ! (fmt , "<{s}>") , DocTest (ref path , _) => write ! (fmt , "{}" , path . display ()) , InlineAsm (_) => write ! (fmt , "<inline asm>") , } } }
};
}
