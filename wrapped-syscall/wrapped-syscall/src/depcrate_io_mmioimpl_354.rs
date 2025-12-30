// Generated macro for impl_354 (impl)
macro_rules! Depcrate_io_mmioimpl_354 {
() => {
// Module: crate::io::mmio
// Provides: {"impl_354"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] impl Io for Mmio < u32 > { type Value = u32 ; fn read (& self) -> Self :: Value { unsafe { let value : Self :: Value ; let ptr : * const Self :: Value = ptr :: addr_of ! (self . value) . cast :: < Self :: Value > () ; core :: arch :: asm ! ("mov {:e}, [{}]" , out (reg) value , in (reg) ptr) ; value } } fn write (& mut self , value : Self :: Value) { unsafe { let ptr : * mut Self :: Value = ptr :: addr_of_mut ! (self . value) . cast :: < Self :: Value > () ; core :: arch :: asm ! ("mov [{}], {:e}" , in (reg) ptr , in (reg) value ,) ; } } }
};
}
