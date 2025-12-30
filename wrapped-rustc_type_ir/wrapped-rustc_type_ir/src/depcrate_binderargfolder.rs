// Generated macro for ArgFolder (struct)
macro_rules! Depcrate_binderArgFolder {
() => {
// Module: crate::binder
// Provides: {"ArgFolder"}
// Dependencies: {}
struct ArgFolder < 'a , I : Interner > { cx : I , args : & 'a [I :: GenericArg] , # [doc = " Number of region binders we have passed through while doing the instantiation"] binders_passed : u32 , }
};
}
