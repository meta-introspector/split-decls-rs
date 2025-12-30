// Generated macro for impl_198 (impl)
macro_rules! Depcrate_fieldimpl_198 {
() => {
// Module: crate::field
// Provides: {"impl_198"}
// Dependencies: {}
# [cfg (all (tracing_unstable , feature = "valuable"))] # [cfg_attr (docsrs , doc (cfg (all (tracing_unstable , feature = "valuable"))))] impl Value for & '_ dyn valuable :: Valuable { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_value (key , self . as_value ()) } }
};
}
