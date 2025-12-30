// Generated macro for Reg (struct)
macro_rules! Depcrate_genericReg {
() => {
// Module: crate::generic
// Provides: {"Reg"}
// Dependencies: {}
# [doc = " This structure provides volatile access to registers."] # [repr (transparent)] pub struct Reg < REG : RegisterSpec > { register : vcell :: VolatileCell < REG :: Ux > , _marker : marker :: PhantomData < REG > , }
};
}
