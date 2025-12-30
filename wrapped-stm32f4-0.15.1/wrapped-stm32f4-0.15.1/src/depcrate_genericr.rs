// Generated macro for R (struct)
macro_rules! Depcrate_genericR {
() => {
// Module: crate::generic
// Provides: {"R"}
// Dependencies: {}
# [doc = " Register reader."] # [doc = ""] # [doc = " Result of the `read` methods of registers. Also used as a closure argument in the `modify`"] # [doc = " method."] pub struct R < REG : RegisterSpec + ? Sized > { pub (crate) bits : REG :: Ux , _reg : marker :: PhantomData < REG > , }
};
}
