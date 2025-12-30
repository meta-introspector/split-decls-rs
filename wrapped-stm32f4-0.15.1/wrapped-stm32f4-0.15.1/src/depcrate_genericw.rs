// Generated macro for W (struct)
macro_rules! Depcrate_genericW {
() => {
// Module: crate::generic
// Provides: {"W"}
// Dependencies: {}
# [doc = " Register writer."] # [doc = ""] # [doc = " Used as an argument to the closures in the `write` and `modify` methods of the register."] pub struct W < REG : RegisterSpec + ? Sized > { # [doc = "Writable bits"] pub (crate) bits : REG :: Ux , _reg : marker :: PhantomData < REG > , }
};
}
