// Generated macro for impl_355 (impl)
macro_rules! Depcrate_io_mmioimpl_355 {
() => {
// Module: crate::io::mmio
// Provides: {"impl_355"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] impl Io for Mmio < u64 > { type Value = u64 ; fn read (& self) -> Self :: Value { unsafe { let value : Self :: Value ; let ptr : * const Self :: Value = ptr :: addr_of ! (self . value) . cast :: < Self :: Value > () ; core :: arch :: asm ! ("mov {:r}, [{}]" , out (reg) value , in (reg) ptr) ; value } } fn write (& mut self , value : Self :: Value) { unsafe { let ptr : * mut Self :: Value = ptr :: addr_of_mut ! (self . value) . cast :: < Self :: Value > () ; core :: arch :: asm ! ("mov [{}], {:r}" , in (reg) ptr , in (reg) value ,) ; } } }
};
}
