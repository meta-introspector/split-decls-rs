// Generated macro for impl_65 (impl)
macro_rules! Depcrate_errorimpl_65 {
() => {
// Module: crate::error
// Provides: {"impl_65"}
// Dependencies: {}
impl < T : Clone > ErrMode < InputError < T > > { # [doc = " Maps `ErrMode<InputError<T>>` to `ErrMode<InputError<U>>` with the given `F: T -> U`"] pub fn map_input < U : Clone , F > (self , f : F) -> ErrMode < InputError < U > > where F : FnOnce (T) -> U , { match self { ErrMode :: Incomplete (n) => ErrMode :: Incomplete (n) , ErrMode :: Cut (InputError { input }) => ErrMode :: Cut (InputError { input : f (input) }) , ErrMode :: Backtrack (InputError { input }) => { ErrMode :: Backtrack (InputError { input : f (input) }) } } } }
};
}
