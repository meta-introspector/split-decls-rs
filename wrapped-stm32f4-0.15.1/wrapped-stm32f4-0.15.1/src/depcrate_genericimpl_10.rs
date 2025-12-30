// Generated macro for impl_10 (impl)
macro_rules! Depcrate_genericimpl_10 {
() => {
// Module: crate::generic
// Provides: {"impl_10"}
// Dependencies: {}
impl < REG : Readable > Reg < REG > { # [doc = " Reads the contents of a `Readable` register."] # [doc = ""] # [doc = " You can read the raw contents of a register by using `bits`:"] # [doc = " ```ignore"] # [doc = " let bits = periph.reg.read().bits();"] # [doc = " ```"] # [doc = " or get the content of a particular field of a register:"] # [doc = " ```ignore"] # [doc = " let reader = periph.reg.read();"] # [doc = " let bits = reader.field1().bits();"] # [doc = " let flag = reader.field2().bit_is_set();"] # [doc = " ```"] # [inline (always)] pub fn read (& self) -> REG :: Reader { REG :: Reader :: from (R { bits : self . register . get () , _reg : marker :: PhantomData , }) } }
};
}
