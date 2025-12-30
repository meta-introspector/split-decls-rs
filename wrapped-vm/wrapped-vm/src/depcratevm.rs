// Generated macro for Vm (struct)
macro_rules! DepcrateVm {
() => {
// Module: crate
// Provides: {"Vm"}
// Dependencies: {}
# [doc = " A virtual machine-like construct that runs an AST on-the-fly"] pub struct Vm { rules : HashMap < String , OptimizedRule > , listener : Option < ListenerFn > , }
};
}
