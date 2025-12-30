// Generated macro for impl_308 (impl)
macro_rules! Depcrate_typesimpl_308 {
() => {
// Module: crate::types
// Provides: {"impl_308"}
// Dependencies: {}
impl fmt :: Debug for TestFn { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match * self { StaticTestFn (..) => "StaticTestFn(..)" , StaticBenchFn (..) => "StaticBenchFn(..)" , StaticBenchAsTestFn (..) => "StaticBenchAsTestFn(..)" , DynTestFn (..) => "DynTestFn(..)" , DynBenchFn (..) => "DynBenchFn(..)" , DynBenchAsTestFn (..) => "DynBenchAsTestFn(..)" , }) } }
};
}
